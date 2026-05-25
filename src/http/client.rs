use anyhow::{Context, Result};
use reqwest::{Client, ClientBuilder, redirect::Policy};
use super::request::HttpRequest;
use super::response::HttpResponse;
use super::useragents::UserAgentPool;

pub struct HttpClientConfig {
    pub insecure: bool,
    pub proxy: Option<String>,
    pub user_agent: Option<String>,
    pub follow_redirects: bool,
    pub max_redirects: u32,
}

impl Default for HttpClientConfig {
    fn default() -> Self {
        Self {
            insecure: false,
            proxy: None,
            user_agent: None,
            follow_redirects: false,
            max_redirects: 10,
        }
    }
}

pub struct HttpClient {
    client:   Client,
    ua_pool:  UserAgentPool,
    user_agent: Option<String>,
}

impl HttpClient {
    pub fn new(config: HttpClientConfig) -> Result<Self> {
        let client = Self::build_client(&config)?;
        Ok(Self {
            client,
            ua_pool:  UserAgentPool::full(),
            user_agent: config.user_agent,
        })
    }

    fn build_client(config: &HttpClientConfig) -> Result<Client> {
        let mut builder = ClientBuilder::new()
            .danger_accept_invalid_certs(config.insecure);

        if let Some(ref proxy_url) = config.proxy {
            let parsed = reqwest::Proxy::all(proxy_url)
                .with_context(|| format!("Invalid proxy URL: {}", proxy_url))?;
            builder = builder.proxy(parsed);
        }

        if config.follow_redirects {
            builder = builder.redirect(Policy::limited(config.max_redirects as usize));
        } else {
            builder = builder.redirect(Policy::none());
        }

        builder
            .build()
            .with_context(|| "Failed to build HTTP client")
    }

    pub async fn send(&self, request: HttpRequest) -> Result<HttpResponse> {
        let ua = self.user_agent.as_deref().unwrap_or_else(|| self.ua_pool.next());
        let (accept, accept_lang, accept_enc) = UserAgentPool::accept_headers_for(ua);

        let mut req = self.client.request(request.method.clone(), request.url.as_str())
            .header("User-Agent",      ua)
            .header("Accept",          accept)
            .header("Accept-Language", accept_lang)
            .header("Accept-Encoding", accept_enc);

        for (key, value) in &request.headers {
            req = req.header(key, value);
        }
        if let Some(body) = &request.body {
            req = req.body(body.clone());
        }
        let response = req
            .send()
            .await
            .with_context(|| format!("Failed to send request to {}", request.url))?;
        HttpResponse::from_reqwest(response).await
    }

    pub async fn get(&self, url: &str) -> Result<HttpResponse> {
        self.send(HttpRequest::get(url)).await
    }

    pub async fn post(&self, url: &str, body: &str) -> Result<HttpResponse> {
        self.send(HttpRequest::post(url, body)).await
    }
}
