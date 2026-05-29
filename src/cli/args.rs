use clap::Parser;

use crate::cli::parser::Parser as ArgParser;

#[derive(Parser, Debug, Clone)]
#[command(name = "oxide")]
#[command(author, version = "8.5.0", about = "OXIDE Community Edition — Open eXtensible Intelligence & Detection Engine", long_about = None)]
pub struct CliArgs {
    #[arg(short, long, help = "Target URL to break... err, scan (up to 3 with --multiattack)")]
    pub url: Vec<String>,

    #[arg(short, long, help = "Concurrent threads (1-100). Higher = faster but more aggressive", default_value_t = 20)]
    pub threads: usize,

    #[arg(long, help = "How hard to hit it (1-100, higher = more aggressive. Try at your own risk)", default_value_t = 50)]
    pub exploitation_level: u8,

    #[arg(long, help = "Max payloads before the server cries uncle", default_value_t = 50)]
    pub payload_limit: usize,

    #[arg(short, long, help = "Where to write your bug report masterpiece")]
    pub output: Option<String>,

    #[arg(short, long, help = "Output format (json, html, csv, xml). We recommend json for machines, html for your manager", default_value_t = String::from("json"))]
    pub format: String,

    #[arg(long, help = "Pretend to be someone else (User-Agent)")]
    pub user_agent: Option<String>,

    #[arg(long, help = "Cookies to sweeten the request")]
    pub cookie: Option<String>,

    #[arg(long, help = "Extra headers to confuse... err, inform the server")]
    pub header: Vec<String>,

    #[arg(short, long, help = "Talk more. Like, a LOT more")]
    pub verbose: bool,

    #[arg(long, help = "Slow down, turbo (requests per second)", default_value_t = 0)]
    pub rate_limit: u64,

    #[arg(long, help = "Follow redirects like a lost puppy")]
    pub follow_redirects: bool,

    #[arg(long, help = "How many redirects before we get dizzy", default_value_t = 10)]
    pub max_redirects: u32,

    #[arg(long, help = "Ignore SSL cert errors (living dangerously)")]
    pub insecure: bool,

    #[arg(long, help = "Route traffic through a middleman (proxy URL)")]
    pub proxy: Option<String>,

    #[arg(long, help = "Pick your weapons (modules: all, engine, static, agent, body, fingerprint, tls, common, cors, creds, insta, session, sqli, xss, lfi, db-fingerprint)")]
    pub modules: Option<String>,

    #[arg(long, help = "How deep the rabbit hole goes (crawl depth)", default_value_t = 3)]
    pub crawl_depth: u8,

    #[arg(long, help = "Max pages to spider before getting bored", default_value_t = 100)]
    pub max_urls: usize,

    #[arg(long, help = "Shhh... be very very quiet (less noise)")]
    pub silent_mode: bool,

    #[arg(long, help = "Download sensitive files when found (database dumps, configs, etc)")]
    pub download: bool,

    #[arg(long, help = "Modules to skip (they're on vacation)")]
    pub exclude: Option<String>,

    #[arg(long, help = "Enable zero-day vulnerability detection. Call a lawyer if you find a 0-day — don't tell others (experimental)")]
    pub zeroday: bool,

    #[arg(long, help = "Active TCP fingerprinting via raw packets (requires sudo). Pokes the server with a stick to see what it runs")]
    pub active: bool,

    #[arg(long, help = "Train zero-day ML classifier by scanning with all modules and collecting labeled response data")]
    pub train: bool,

    #[arg(long, help = "Enable Instagram OSINT module — follower count, private profile detection, media download")]
    pub insta: bool,

    #[arg(long, help = "Enable session hijack testing — cookie flags, fixation, predictability")]
    pub session: bool,

    #[arg(long, help = "Multi-attack mode: scan up to 3 targets concurrently with adaptive thread distribution")]
    pub multiattack: bool,

    #[arg(long, help = "Scan duration in seconds (0 = unlimited). Hard timeout — scan stops when reached", default_value_t = 0)]
    pub duration: u64,

    #[arg(long, help = "List available modules and exit")]
    pub list_modules: bool,
}

impl CliArgs {
    pub fn parse_args() -> anyhow::Result<Self> {
        let mut args = Self::parse();

        if args.url.is_empty() {
            anyhow::bail!("No target URL provided. Use: oxide --url <URL>");
        }

        // Expand .txt file references: -u targets.txt reads lines as URLs
        let mut expanded = Vec::new();
        for u in &args.url {
            if u.ends_with(".txt") && std::path::Path::new(u).exists() {
                let content = std::fs::read_to_string(u)
                    .map_err(|e| anyhow::anyhow!("Failed to read target file '{}': {}", u, e))?;
                for line in content.lines() {
                    let line = line.trim();
                    if !line.is_empty() {
                        expanded.push(line.to_string());
                    }
                }
            } else {
                expanded.push(u.clone());
            }
        }
        args.url = expanded;

        // Validate and clamp threads to safe range (1-100)
        if args.threads > 100 {
            eprintln!("[WARN] threads clamped to 100 (was {})", args.threads);
            args.threads = 100;
        }
        if args.threads < 1 {
            eprintln!("[WARN] threads raised to 1 (was {})", args.threads);
            args.threads = 1;
        }

        // Clamp exploitation level
        if args.exploitation_level > 100 {
            eprintln!("[WARN] exploitation_level clamped to 100 (was {})", args.exploitation_level);
            args.exploitation_level = 100;
        }

        // Clamp payload limit
        if args.payload_limit > 500 {
            eprintln!("[WARN] payload_limit clamped to 500 (was {})", args.payload_limit);
            args.payload_limit = 500;
        }

        // Clamp crawl depth
        if args.crawl_depth > 10 {
            eprintln!("[WARN] crawl_depth clamped to 10 (was {})", args.crawl_depth);
            args.crawl_depth = 10;
        }

        // Clamp max_urls
        if args.max_urls > 10_000 {
            eprintln!("[WARN] max_urls clamped to 10000 (was {})", args.max_urls);
            args.max_urls = 10_000;
        }

        Ok(args)
    }

    pub fn get_modules(&self) -> Vec<String> {
        match &self.modules {
            Some(m) => ArgParser::parse_modules(m),
            None => vec!["all".to_string()],
        }
    }

    pub fn get_excluded(&self) -> Vec<String> {
        match &self.exclude {
            Some(e) => e.split(',').map(|s| s.trim().to_string()).collect(),
            None => vec![],
        }
    }

    pub fn target_url(&self) -> &str {
        if self.url.is_empty() {
            panic!("No target URL provided — use --url <target>");
        }
        &self.url[0]
    }

    pub fn target_count(&self) -> usize {
        self.url.len()
    }

    pub fn multiattack_enabled(&self) -> bool {
        self.multiattack && self.url.len() > 1
    }
}
