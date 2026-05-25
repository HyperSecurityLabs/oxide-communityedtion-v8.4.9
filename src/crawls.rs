use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use url::Url;

use crate::http::client::{HttpClient, HttpClientConfig};
use crate::http::request::HttpRequest;
use crate::http::response::HttpResponse;
use crate::utils::url::UrlUtil;

const REQUEST_TIMEOUT_SECS: u64 = 30;

pub struct WebCrawler {
    client: HttpClient,
    client_config: HttpClientConfig,
    max_depth: usize,
    max_pages: usize,
    visited: HashSet<String>,
    queue: VecDeque<(String, usize)>,
    discovered_urls: Vec<String>,
    all_linked_urls: Vec<String>,
    forms: Vec<FormData>,
    links: Vec<LinkData>,
    comments: Vec<String>,
    scripts: Vec<String>,
    rate_limit: Option<Duration>,
    robots_disallowed: HashMap<String, Vec<String>>,
}

#[derive(Clone, Debug)]
pub struct FormData {
    pub url: String,
    pub method: String,
    pub action: String,
    pub inputs: Vec<InputField>,
}

#[derive(Clone, Debug)]
pub struct InputField {
    pub name: String,
    pub input_type: String,
    pub value: Option<String>,
}

#[derive(Clone, Debug)]
pub struct LinkData {
    pub from: String,
    pub to: String,
    pub text: String,
}

#[derive(Debug)]
pub struct CrawlResult {
    pub urls: Vec<String>,
    pub all_linked_urls: Vec<String>,
    pub forms: Vec<FormData>,
    pub links: Vec<LinkData>,
    pub comments: Vec<String>,
    pub scripts: Vec<String>,
}

impl CrawlResult {
    pub fn get_forms_by_method(&self, method: &str) -> Vec<&FormData> {
        self.forms.iter().filter(|f| f.method.eq_ignore_ascii_case(method)).collect()
    }

    pub fn get_all_link_texts(&self) -> Vec<&String> {
        self.links.iter().map(|l| &l.text).filter(|t| !t.is_empty()).collect()
    }

    /// Scan comments for patterns that look like credentials or internal paths.
    /// Returns (comment_text, reason) pairs.
    pub fn suspicious_comments(&self) -> Vec<(&String, &'static str)> {
        let patterns: &[(&str, &str)] = &[
            ("password", "possible credential"),
            ("passwd",   "possible credential"),
            ("secret",   "possible secret"),
            ("token",    "possible token"),
            ("api_key",  "possible API key"),
            ("todo",     "developer note"),
            ("fixme",    "developer note"),
            ("hack",     "developer note"),
            ("/etc/",    "internal path"),
            ("192.168.", "internal IP"),
            ("10.0.",    "internal IP"),
        ];
        self.comments.iter().filter_map(|c| {
            let cl = c.to_lowercase();
            patterns.iter().find(|(p, _)| cl.contains(p)).map(|(_, reason)| (c, *reason))
        }).collect()
    }

    /// Extract potential API endpoints from inline scripts.
    pub fn script_endpoints(&self) -> Vec<String> {
        let Ok(re) = regex::Regex::new(r#"["'](/(?:api|v\d|rest|graphql)[^"'\s]*)"#) else {
            return Vec::new();
        };
        self.scripts.iter().flat_map(|s| {
            re.captures_iter(s)
                .filter_map(|c| c.get(1).map(|m| m.as_str().to_string()))
        }).collect()
    }
}

impl WebCrawler {
    pub fn new(config: HttpClientConfig, max_depth: usize, max_pages: usize) -> Result<Self> {
        let client_config = config.clone();
        let client = HttpClient::new(config)?;
        Ok(Self {
            client,
            client_config,
            max_depth,
            max_pages,
            visited: HashSet::new(),
            queue: VecDeque::new(),
            discovered_urls: Vec::new(),
            all_linked_urls: Vec::new(),
            forms: Vec::new(),
            links: Vec::new(),
            comments: Vec::new(),
            scripts: Vec::new(),
            rate_limit: None,
            robots_disallowed: HashMap::new(),
        })
    }

    pub fn set_rate_limit(&mut self, delay: Duration) {
        self.rate_limit = Some(delay);
    }

    pub async fn crawl(&mut self, start_url: &str) -> Result<CrawlResult> {
        self.queue.push_back((start_url.to_string(), 0));
        let mut page_count = 0;
        let start = std::time::Instant::now();

        // Fetch robots.txt before crawling (non-fatal if unavailable)
        let _ = self.fetch_robots_txt(start_url).await;

        while let Some((url, depth)) = self.queue.pop_front() {
            if self.visited.contains(&url) || depth > self.max_depth || page_count >= self.max_pages {
                continue;
            }

            // Check robots.txt
            if !self.is_allowed_by_robots(&url) {
                continue;
            }

            self.visited.insert(url.clone());
            page_count += 1;

            // Rate limiting
            if let Some(delay) = self.rate_limit {
                tokio::time::sleep(delay).await;
            }

            let request = HttpRequest::get(&url);
            let url_display = if url.len() > 55 { format!("..{}", &url[url.len()-53..]) } else { url.clone() };
            let forms_before = self.forms.len();
            let links_before = self.links.len();
            let depth_val = depth;

            let spin_stop = Arc::new(AtomicBool::new(false));
            let s = spin_stop.clone();
            let url_s = url_display.clone();
            let start_for_spinner = start;
            tokio::spawn(async move {
                let mut idx = 0usize;
                while !s.load(Ordering::Relaxed) {
                    let elapsed = start_for_spinner.elapsed().as_secs();
                    let frame = match idx % 10 {
                        0 => "⠋", 1 => "⠙", 2 => "⠹", 3 => "⠸", 4 => "⠼",
                        5 => "⠴", 6 => "⠦", 7 => "⠧", 8 => "⠇", 9 => "⠏",
                        _ => "⠋",
                    };
                    idx += 1;
                    print!("\r  \x1B[90m[*]\x1B[0m \x1B[93m{}\x1B[0m fetching  \x1B[90mdepth:{}\x1B[0m  \x1B[90m{}\x1B[0m  \x1B[90m[{:02}:{:02}]\x1B[0m",
                        frame, depth_val, url_s, elapsed / 60, elapsed % 60);
                    let _ = std::io::Write::flush(&mut std::io::stdout());
                    tokio::time::sleep(Duration::from_millis(120)).await;
                }
            });

            let send_fut = self.client.send(request);
            let result = tokio::time::timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS), send_fut).await;

            spin_stop.store(true, Ordering::Relaxed);
            // Brief yield to let spinner task print its last frame
            tokio::time::sleep(Duration::from_millis(10)).await;

            match result {
                Ok(Ok(response)) => {
                    self.process_response(&url, &response, depth).await?;
                    let new_forms = self.forms.len() - forms_before;
                    let new_links = self.links.len() - links_before;
                    let elapsed = start.elapsed().as_secs();
                    let status = response.status;
                    let size_str = if response.body.len() >= 1_048_576 {
                        format!("{:.1}MB", response.body.len() as f64 / 1_048_576.0)
                    } else if response.body.len() >= 1_024 {
                        format!("{:.1}KB", response.body.len() as f64 / 1_024.0)
                    } else {
                        format!("{}B", response.body.len())
                    };
                    print!("\r\x1B[2K");
                    println!("  \x1B[90m[*]\x1B[0m {} {}  \x1B[90mdepth:{} forms:{} links:{}\x1B[0m  \x1B[90m{}\x1B[0m  \x1B[90m[{:02}:{:02}]\x1B[0m",
                        status, size_str,
                        depth, new_forms, new_links, url_display,
                        elapsed / 60, elapsed % 60);
                }
                Ok(Err(_)) => {
                    let elapsed = start.elapsed().as_secs();
                    print!("\r\x1B[2K");
                    println!("  \x1B[90m[*]\x1B[0m \x1B[91mERR\x1B[0m  \x1B[90mdepth:{}\x1B[0m  \x1B[90m{}\x1B[0m  \x1B[90m[{:02}:{:02}]\x1B[0m",
                        depth, url_display, elapsed / 60, elapsed % 60);
                }
                Err(_) => {
                    let elapsed = start.elapsed().as_secs();
                    print!("\r\x1B[2K");
                    println!("  \x1B[90m[*]\x1B[0m \x1B[91mTIMEOUT\x1B[0m  \x1B[90mdepth:{}\x1B[0m  \x1B[90m{}\x1B[0m  \x1B[90m[{:02}:{:02}]\x1B[0m",
                        depth, url_display, elapsed / 60, elapsed % 60);
                }
            }
        }

        let result = CrawlResult {
            urls:           self.discovered_urls.clone(),
            all_linked_urls: self.all_linked_urls.clone(),
            forms:          self.forms.clone(),
            links:          self.links.clone(),
            comments:       self.comments.clone(),
            scripts:        self.scripts.clone(),
        };
        let total = result.urls.len();
        println!("  \x1B[38;2;255;140;0m[+]\x1B[0m Crawl complete: {} pages, {} URLs, {} forms, {} links",
            page_count, total, self.forms.len(), self.links.len());
        Ok(result)
    }

    async fn process_response(&mut self, url: &str, response: &HttpResponse, depth: usize) -> Result<()> {
        if !response.is_success() {
            return Ok(());
        }

        self.discovered_urls.push(url.to_string());
        let body = &response.body;

        self.extract_links(url, body, depth).await?;
        self.extract_forms(url, body).await?;

        // Collect comments and scripts into the crawler state
        let mut found_comments = self.extract_comments(body);
        let mut found_scripts  = self.extract_scripts(body);
        self.comments.append(&mut found_comments);
        self.scripts.append(&mut found_scripts);

        Ok(())
    }

    async fn extract_links(&mut self, base_url: &str, body: &str, depth: usize) -> Result<()> {
        let tag_re = regex::Regex::new(r"<[^>]*>")
            .map_err(|e| anyhow::anyhow!("tag regex: {}", e))?;

        // Match <a href="...">, <a href='...'>, and <a href=...> (unquoted)
        let href_re = regex::Regex::new(
            r#"(?x)
            <(a|link|area)\s
            [^>]*?
            href\s*=\s*
            (?:
                "([^"]*)"   |   # double-quoted
                '([^']*)'   |   # single-quoted
                ([^>\s]+)       # unquoted
            )
            "#,
        ).map_err(|e| anyhow::anyhow!("href regex: {}", e))?;

        for cap in href_re.captures_iter(body) {
            let href = cap.get(2).or_else(|| cap.get(3)).or_else(|| cap.get(4))
                .map(|m| m.as_str())
                .unwrap_or("");
            if href.is_empty() || href.starts_with('#') || href.starts_with("javascript:") || href.starts_with("mailto:") {
                continue;
            }

            let absolute_url = match self.resolve_url(base_url, href) {
                Ok(u) => u,
                Err(_) => continue,
            };
            self.all_linked_urls.push(absolute_url.clone());
            if self.is_same_domain(base_url, &absolute_url) && !self.visited.contains(&absolute_url) {
                self.links.push(LinkData {
                    from: base_url.to_string(),
                    to: absolute_url.clone(),
                    text: String::new(),
                });
                self.queue.push_back((absolute_url, depth + 1));
            }
        }

        // Also extract <a>...</a> link text for display purposes
        let a_re = regex::Regex::new(r#"<a[^>]*href=["']([^"']+)["'][^>]*>(.*?)</a>"#)
            .map_err(|e| anyhow::anyhow!("a tag regex: {}", e))?;
        for cap in a_re.captures_iter(body) {
            let href = match cap.get(1) { Some(m) => m.as_str(), None => continue };
            let raw_text = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let link_text = tag_re.replace_all(raw_text, "").to_string();
            if link_text.is_empty() { continue; }
            if let Ok(absolute_url) = self.resolve_url(base_url, href) {
                if self.is_same_domain(base_url, &absolute_url) {
                    // update link text in existing entry
                    if let Some(link) = self.links.iter_mut().find(|l| l.to == absolute_url) {
                        link.text = link_text;
                    }
                }
            }
        }
        Ok(())
    }

    async fn extract_forms(&mut self, url: &str, body: &str) -> Result<()> {
        let form_re   = regex::Regex::new(r#"(?s)<form[^>]*>.*?</form>"#)
            .map_err(|e| anyhow::anyhow!("form regex: {}", e))?;
        let action_re = regex::Regex::new(r#"action=["']([^"']*)["']"#)
            .map_err(|e| anyhow::anyhow!("action regex: {}", e))?;
        let method_re = regex::Regex::new(r#"method=["']([^"']*)["']"#)
            .map_err(|e| anyhow::anyhow!("method regex: {}", e))?;
        let input_re  = regex::Regex::new(r#"<input[^>]*>"#)
            .map_err(|e| anyhow::anyhow!("input regex: {}", e))?;
        let name_re   = regex::Regex::new(r#"name=["']([^"']*)["']"#)
            .map_err(|e| anyhow::anyhow!("name regex: {}", e))?;
        let type_re   = regex::Regex::new(r#"type=["']([^"']*)["']"#)
            .map_err(|e| anyhow::anyhow!("type regex: {}", e))?;
        let value_re  = regex::Regex::new(r#"value=["']([^"']*)["']"#)
            .map_err(|e| anyhow::anyhow!("value regex: {}", e))?;

        for form_m in form_re.find_iter(body) {
            let form_html = form_m.as_str();

            let action = action_re.captures(form_html)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().to_string())
                .unwrap_or_else(|| url.to_string());

            let method = method_re.captures(form_html)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().to_uppercase())
                .unwrap_or_else(|| "GET".to_string());

            let inputs: Vec<InputField> = input_re.find_iter(form_html).filter_map(|im| {
                let ih = im.as_str();
                let name = name_re.captures(ih)?.get(1)?.as_str().to_string();
                if name.is_empty() { return None; }
                Some(InputField {
                    name,
                    input_type: type_re.captures(ih)
                        .and_then(|c| c.get(1))
                        .map(|m| m.as_str().to_string())
                        .unwrap_or_else(|| "text".to_string()),
                    value: value_re.captures(ih)
                        .and_then(|c| c.get(1))
                        .map(|m| m.as_str().to_string()),
                })
            }).collect();

            self.forms.push(FormData { url: url.to_string(), method, action, inputs });
        }
        Ok(())
    }

    /// Extract HTML comments — returns owned strings stored in `self.comments`.
    fn extract_comments(&self, body: &str) -> Vec<String> {
        let Ok(re) = regex::Regex::new(r"<!--([\s\S]*?)-->") else { return Vec::new() };
        re.captures_iter(body)
            .filter_map(|c| c.get(1).map(|m| m.as_str().trim().to_string()))
            .filter(|s| !s.is_empty())
            .collect()
    }

    /// Extract inline `<script>` blocks — returns owned strings stored in `self.scripts`.
    fn extract_scripts(&self, body: &str) -> Vec<String> {
        let Ok(re) = regex::Regex::new(r"(?s)<script[^>]*>(.*?)</script>") else { return Vec::new() };
        re.captures_iter(body)
            .filter_map(|c| c.get(1).map(|m| m.as_str().trim().to_string()))
            .filter(|s| !s.is_empty())
            .collect()
    }

    fn resolve_url(&self, base: &str, relative: &str) -> Result<String> {
        let base_url = Url::parse(base).with_context(|| format!("Invalid base URL: {}", base))?;
        let resolved = base_url.join(relative)
            .with_context(|| format!("Failed to join: {} + {}", base, relative))?;
        Ok(resolved.to_string())
    }

    fn is_same_domain(&self, url1: &str, url2: &str) -> bool {
        let parsed1 = Url::parse(url1);
        let parsed2 = Url::parse(url2);
        match (parsed1, parsed2) {
            (Ok(u1), Ok(u2)) => {
                let d1 = UrlUtil::extract_domain(&u1);
                let d2 = UrlUtil::extract_domain(&u2);
                !d1.is_empty() && d1 == d2
            }
            _ => false,
        }
    }

    pub fn get_forms(&self) -> &Vec<FormData>  { &self.forms }

    pub fn get_forms_by_method(&self, method: &str) -> Vec<&FormData> {
        self.forms.iter().filter(|f| f.method.eq_ignore_ascii_case(method)).collect()
    }

    pub fn get_links_with_text(&self) -> Vec<(&String, &String, &String)> {
        self.links.iter().map(|l| (&l.from, &l.to, &l.text)).collect()
    }

    async fn fetch_robots_txt(&mut self, base_url: &str) -> Result<()> {
        let parsed = Url::parse(base_url)?;
        let robots_url = format!("{}://{}/robots.txt", parsed.scheme(), parsed.host_str().unwrap_or(""));
        let request = HttpRequest::get(&robots_url);
        if let Ok(response) = self.client.send(request).await {
            let domain = parsed.host_str().unwrap_or("").to_string();
            let disallowed: Vec<String> = response.body.lines()
                .filter_map(|line| {
                    let trimmed = line.trim();
                    if trimmed.to_lowercase().starts_with("disallow") {
                        trimmed.split(':').nth(1).map(|p| p.trim().to_string())
                    } else {
                        None
                    }
                })
                .filter(|p| !p.is_empty())
                .collect();
            self.robots_disallowed.insert(domain, disallowed);
        }
        Ok(())
    }

    fn is_allowed_by_robots(&self, url: &str) -> bool {
        let parsed = match Url::parse(url) {
            Ok(u) => u,
            Err(_) => return true,
        };
        let domain = match parsed.host_str() {
            Some(d) => d.to_string(),
            None => return true,
        };
        let Some(disallowed) = self.robots_disallowed.get(&domain) else {
            return true;
        };
        let path = parsed.path();
        !disallowed.iter().any(|d| path.starts_with(d))
    }
}

impl Clone for WebCrawler {
    fn clone(&self) -> Self {
        let client = HttpClient::new(self.client_config.clone())
            .expect("Critical: unable to clone HTTP client in WebCrawler::clone");
        Self {
            client,
            client_config: self.client_config.clone(),
            max_depth: self.max_depth,
            max_pages: self.max_pages,
            visited: self.visited.clone(),
            queue: self.queue.clone(),
            discovered_urls: self.discovered_urls.clone(),
            all_linked_urls: self.all_linked_urls.clone(),
            forms: self.forms.clone(),
            links: self.links.clone(),
            comments: self.comments.clone(),
            scripts: self.scripts.clone(),
            rate_limit: self.rate_limit,
            robots_disallowed: self.robots_disallowed.clone(),
        }
    }
}
