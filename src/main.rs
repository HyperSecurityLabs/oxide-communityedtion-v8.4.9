use colored::Colorize;
use std::process;
use std::time::Instant;
///! Oxidation Reaction Main control


mod crawls;
mod hybrid;
mod agent;
#[cfg(target_os = "linux")]
mod recon;
mod zero_day;

pub use oxide::cli;
pub use oxide::core;
pub use oxide::http;
pub use oxide::payload;
pub use oxide::detection;
pub use oxide::report;
pub use oxide::utils;

use crate::cli::args::CliArgs;
use crate::cli::colors;
use crate::cli::colors::Colors;
use crate::cli::config::Config;
use crate::http::client::{HttpClient, HttpClientConfig};
use crate::cli::display::{
    COL_DIM, COL_INFO, COL_MED,
    OSAKA_JADE, OSAKA_JADE_B, LAVENDER, LAVENDER_BLUE, LAVENDER_B, LAVENDER_BLUE_B,
};
use crate::cli::output::Output;
use crate::cli::parser::Parser;
use crate::cli::spinner::Spinner;
use crate::utils::time::TimeUtil;
use hybrid::HybridScanner;
use core::engine::ScanEngine;

fn tc(s: &str, (r, g, b): (u8, u8, u8)) -> String {
    s.truecolor(r, g, b).to_string()
}

fn print_banner() {
    use crate::cli::display::{
        OSAKA_JADE, OSAKA_JADE_B, LAVENDER, LAVENDER_BLUE,
    };
    let line1 = format!("◆ Hypersecurity Offensive Labs  |  OXIDE Community Edition v8.5.0");
    let line2 = format!(">> Open eXtensible Intelligence & Detection Engine — Community Edition << osaka-jade");
    let max_w = line1.len().max(line2.len()) + 3;
    let p = "─".repeat(max_w);
    let pad = "  ";
    println!();
    println!("{}", tc("   ____ _  __ ________  ______", OSAKA_JADE_B));
    println!("{}", tc("  / __ \\ |/ //  _/ __ \\/ ____/", OSAKA_JADE));
    println!("{}", tc(" / / / /   / / // / / / __/", OSAKA_JADE));
    println!("{}", tc("/ /_/ /   |_/ // /_/ / /___", OSAKA_JADE));
    println!("{}", tc("\\____/_/|_/___/_____/_____/", OSAKA_JADE_B));
    println!();
    println!("{}", tc(&format!("╭{}╮", p), LAVENDER));
    println!("{}{}{}{}",
        tc("│", LAVENDER),
        tc(pad, LAVENDER),
        tc(&line1, LAVENDER),
        tc(" │", LAVENDER));
    println!("{}{}{}{}",
        tc("│", LAVENDER),
        tc(pad, LAVENDER),
        tc(&line2, LAVENDER_BLUE),
        tc(" │", LAVENDER));
    println!("{}", tc(&format!("╰{}╯", p), LAVENDER));
    println!("{}", tc("----------------------------", OSAKA_JADE));
    println!("{} {}:{}", tc("Author", OSAKA_JADE), tc("khaninkali", OSAKA_JADE_B), tc("Lyara", LAVENDER));
    println!("{} {}", tc("↳", OSAKA_JADE), tc("https://hypersecurity_offsec", OSAKA_JADE_B));
    println!("{} {}", tc("↳", OSAKA_JADE), tc("oxide --url <target> --modules all", OSAKA_JADE));
    println!("{} {}", tc("↳", OSAKA_JADE), tc("oxide -u https://example.com --duration 120", OSAKA_JADE));
    println!("{} {}", tc("↳", OSAKA_JADE), tc("oxide -u https://example.com --fuzz --exploit 80", OSAKA_JADE));
    println!("{} {}", tc("↳", OSAKA_JADE), tc("oxide --url <target> --modules xss,sqli --threads 50", OSAKA_JADE));
    println!("{} {}", tc("↳", OSAKA_JADE), tc("oxide --list-modules", OSAKA_JADE));
    println!("{} {}", tc("↳", OSAKA_JADE), tc("oxide -u targets.txt --multiattack --output json --verbose", OSAKA_JADE));
    println!("{}", tc("----------------------------", OSAKA_JADE));
    println!();
}

/// Resolve the IP address(es) for a hostname using tokio's built-in DNS.
/// Returns a deduplicated list of IP strings, or an empty vec on failure.

async fn resolve_ip(host: &str) -> Vec<String> {
    use std::collections::BTreeSet;
    // lookup_host needs a host:port pair
    let addr = format!("{}:80", host);
    match tokio::net::lookup_host(addr).await {
        Ok(addrs) => {
            let ips: BTreeSet<String> = addrs.map(|a| a.ip().to_string()).collect();
            ips.into_iter().collect()
        }
        Err(_) => Vec::new(),
    }
}

async fn print_scan_info(args: &CliArgs) {
    let tc = |s: &str, (r, g, b): (u8, u8, u8)| s.truecolor(r, g, b).to_string();
    use crate::cli::display::{
        OSAKA_JADE_B, LAVENDER, LAVENDER_B, LAVENDER_BLUE,
        COL_HIGH,
    };

    Output::print_header("Target Information");
    if args.multiattack_enabled() {
        println!("  {} {}  {} {}",
            tc("▸", OSAKA_JADE_B), tc("Multi-Attack", LAVENDER_B).bold(),
            tc("→", LAVENDER), tc(&format!("{} targets", args.target_count()), LAVENDER_BLUE));
        let per_target = (args.threads / args.target_count()).max(1);
        for (i, url) in args.url.iter().enumerate() {
            let clean = Parser::ensure_http(url);
            let _host = url::Url::parse(&clean)
                .ok()
                .and_then(|u| u.host_str().map(|h| h.to_string()))
                .unwrap_or_default();
            println!("  {} {}  {} {}  {} {}",
                tc("▸", OSAKA_JADE_B), tc(&format!("Target {}", i + 1), LAVENDER_B).bold(),
                tc("→", LAVENDER), tc(&clean, LAVENDER),
                tc("≈", OSAKA_JADE_B), tc(&format!("{} thr", per_target), OSAKA_JADE_B));
        }
        println!("  {} {}  {} {}  {} {}",
            tc("▸", OSAKA_JADE_B), tc("Threads", LAVENDER_B).bold(),
            tc("→", LAVENDER), tc(&format!("{} total", args.threads), OSAKA_JADE_B),
            tc("·", OSAKA_JADE_B), tc(&format!("{}s duration", args.duration), LAVENDER));
    } else {
        let clean = Parser::ensure_http(args.target_url());
        let host = url::Url::parse(&clean)
            .ok()
            .and_then(|u| u.host_str().map(|h| h.to_string()))
            .unwrap_or_default();
        let ips = resolve_ip(&host).await;
        let ip_display = if ips.is_empty() {
            tc("unresolved", COL_HIGH)
        } else {
            tc(&ips.join(", "), OSAKA_JADE_B)
        };
        println!("  {} {}  {} {}",
            tc("▸", OSAKA_JADE_B), tc("Target", LAVENDER_B).bold(),
            tc("→", LAVENDER), args.target_url().truecolor(LAVENDER.0, LAVENDER.1, LAVENDER.2).bold());
        println!("  {} {}  {} {}",
            tc("▸", OSAKA_JADE_B), tc("IP", LAVENDER_B).bold(),
            tc("→", LAVENDER), ip_display);
        println!("  {} {}  {} {}  {} {}",
            tc("▸", OSAKA_JADE_B), tc("Threads", LAVENDER_B).bold(),
            tc("→", LAVENDER), tc(&args.threads.to_string(), OSAKA_JADE_B),
            tc("·", OSAKA_JADE_B), tc(&format!("{}s duration", args.duration), LAVENDER));
    }

    let modules = args.get_modules();
    let module_line: Vec<String> = modules.iter().map(|m| tc(m, LAVENDER_BLUE)).collect();
    println!("  {} {}  {} {}",
        tc("▸", OSAKA_JADE_B), tc("Modules", LAVENDER_B).bold(),
        tc("→", LAVENDER), module_line.join(tc(" │ ", LAVENDER).as_str()));

    if let Some(output) = &args.output {
        println!("  {} {}  {} {}",
            tc("▸", OSAKA_JADE_B), tc("Output", LAVENDER_B).bold(),
            tc("→", LAVENDER), tc(output, OSAKA_JADE_B));
    }

    if args.verbose  { println!("  {} {}", tc("▸", OSAKA_JADE_B), tc("Verbose mode", OSAKA_JADE_B)); }
    if args.insecure { println!("  {} {}", tc("▸", OSAKA_JADE_B), tc("SSL verification disabled", COL_HIGH)); }
    if args.zeroday  { println!("  {} {}", tc("▸", OSAKA_JADE_B), tc("Zero-day detection", OSAKA_JADE_B)); }
    if args.train    { println!("  {} {}", tc("▸", OSAKA_JADE_B), tc("Training mode", COL_HIGH)); }
    if args.insta    { println!("  {} {}", tc("▸", OSAKA_JADE_B), tc("Instagram OSINT", LAVENDER_BLUE_B)); }
    if args.session  { println!("  {} {}", tc("▸", OSAKA_JADE_B), tc("Session hijack testing", LAVENDER_BLUE)); }

    if !args.header.is_empty() {
        Output::print_section("Custom Headers");
        for header in &args.header {
            match Parser::parse_header(header) {
                Ok((key, value)) => println!("    {}: {}",
                    tc(&key, OSAKA_JADE_B), tc(&value, LAVENDER_B)),
                Err(e) => println!("    Invalid header '{}': {}", header, e),
            }
        }
    }

    if let Some(cookie) = &args.cookie {
        Output::print_section("Cookies");
        for (key, value) in &Parser::parse_cookie(cookie) {
            println!("    {}: {}",
                tc(&key, OSAKA_JADE_B), tc(&value, LAVENDER_B));
        }
    }

    Output::print_line();
    println!();
}

#[tokio::main]
async fn main() {
    print_banner();
    
    let args = match CliArgs::parse_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{} {}", "[ERROR]".red().bold(), e);
            process::exit(1);
        }
    };

    if args.list_modules {
        let modules = [
            ("all", "Run all modules (default)"),
            ("fingerprint", "Target fingerprinting — WAF, server, OS detection"),
            ("crawl", "Crawl target for URLs, forms, scripts, comments"),
            ("fuzz", "Fuzz all parameters with injection payloads (SQLi, XSS, LFI, CMDi, NoSQL, SSTI)"),
            ("sqli", "SQL injection detection"),
            ("xss", "Cross-site scripting detection"),
            ("lfi", "Local file inclusion detection"),
            ("tls", "TLS/SSL configuration assessment"),
            ("cors", "CORS misconfiguration scanning"),
            ("common", "Common paths and files (Nikto-style)"),
            ("creds", "Default credentials testing"),
            ("filter", "Content filter — sensitive data exposure (API keys, tokens, passwords)"),
            ("insta", "Instagram OSINT — follower count, profile detection, media download"),
            ("session", "Session hijack testing — cookie flags, fixation, predictability"),
            ("zeroday", "ML-based zero-day anomaly detection"),
            ("static", "Static path scanning"),
            ("agent", "Agent-based parallel vulnerability scanning"),
            ("body", "Response body scanning for signatures"),
            ("parameter-discovery", "Parameter fuzzing and discovery"),
            ("engine", "Legacy ScanEngine (replaced by hybrid)"),
        ];
        println!("\n  {} Available modules:",
            "◆".truecolor(OSAKA_JADE_B.0, OSAKA_JADE_B.1, OSAKA_JADE_B.2));
        for (name, desc) in modules {
            println!("  {}  {}  — {}",
                "▸".truecolor(OSAKA_JADE.0, OSAKA_JADE.1, OSAKA_JADE.2),
                name.truecolor(LAVENDER_BLUE.0, LAVENDER_BLUE.1, LAVENDER_BLUE.2),
                desc.truecolor(LAVENDER.0, LAVENDER.1, LAVENDER.2));
        }
        println!();
        process::exit(0);
    }
    
    // Use TimeUtil for timing
    let start_time = Instant::now();
    let scan_start = TimeUtil::now();
    println!("Scan started at: {}", TimeUtil::format_timestamp(&scan_start));
    println!("Unix timestamp: {}", TimeUtil::unix_timestamp());
    
    // Validate proxy library — binary won't run without it
    crate::http::proxy_loader::ensure_proxy_library();
    
    // Load or create default config
    let config_path = std::path::PathBuf::from("oxide-config.toml");
    let mut config = if config_path.exists() {
        match Config::load(&config_path) {
            Ok(c) => {
                println!("Loaded config from {}", config_path.display());
                c
            }
            Err(e) => {
                println!("Failed to load config: {}, using defaults", e);
                Config::default()
            }
        }
    } else {
        let default_config = Config::default();
        if let Err(e) = default_config.save(&config_path) {
            println!("Failed to save default config: {}", e);
        } else {
            println!("Created default config at {}", config_path.display());
        }
        default_config
    };
    
    // Add custom headers to config
    for header in &args.header {
        if let Ok((key, value)) = Parser::parse_header(header) {
            config.add_header(&key, &value);
        }
    }
    
    // Get and display config headers
    let headers = config.get_headers();
    if !headers.is_empty() {
        println!("Loaded {} custom headers from config", headers.len());
    }
    
    // Validate URL using Parser
    let validated_url = Parser::ensure_http(args.target_url());
    
    // Use Parser::parse_url for strict validation
    match Parser::parse_url(&validated_url) {
        Ok(url) => println!("Valid URL parsed: {}", url),
        Err(e) => {
            eprintln!("{} Invalid URL: {}", "[ERROR]".red().bold(), e);
            process::exit(1);
        }
    }
    
    // Use Parser::is_valid_domain for domain validation
    let clean_url = validated_url.replace("http://", "").replace("https://", "");
    let domain = clean_url.split('/').next().unwrap_or("");
    if !Parser::is_valid_domain(domain) {
        eprintln!("{} Invalid domain: {}", "[ERROR]".red().bold(), domain);
        process::exit(1);
    }
    println!("Domain validation passed: {} (no sneaky redirects detected... yet)", domain);
    
    let funny_messages = vec![
        "Scanning so hard, even the server is nervous...",
        "Looking for bugs like a raccoon in a trash can...",
        "Poking endpoints with a digital stick...",
        "Hunting vulnerabilities like it's a video game...",
        "Scanning for weak spots like a caffeinated pentester...",
        "Making packets do the heavy lifting...",
        "Scanning with the intensity of a midnight coder...",
    ];
    let msg_idx = (scan_start.timestamp() as usize) % funny_messages.len();
    println!("[+] {}", funny_messages[msg_idx].bright_cyan());
    
    println!("Using config: {} threads, {} custom headers", 
        config.threads, headers.len());
    println!("[+] Firing up the engines... hope your firewall is ready");
    
    print_scan_info(&args).await;
    
    // Initialize fingerprint spinner
    let _finger_spin = Spinner::finger_spinner();
    
    // ── Train mode: run all scanners and train zero-day ML classifier ─────
    if args.train {
        println!("{}", "Training mode engaged — indexing all scanners...".bright_green().bold());
        let train_config = HttpClientConfig {
            insecure: args.insecure,
            proxy: args.proxy.clone(),
            user_agent: args.user_agent.clone(),
            follow_redirects: args.follow_redirects,
            max_redirects: args.max_redirects,
        };
        let client = std::sync::Arc::new(HttpClient::new(train_config)
            .expect("Failed to create HTTP client for training"));
        let engine = zero_day::engine::ZeroDayEngine::new();
        let trainer = zero_day::trainer::ZeroDayTrainer::new(
            client, engine, args.target_url(), 120,
        );
        match trainer.run_training().await {
            Ok(()) => {
                println!("{} Training complete!", "[OK]".green().bold());
                process::exit(0);
            }
            Err(e) => {
                eprintln!("{} Training failed: {}", "[ERROR]".red().bold(), e);
                process::exit(1);
            }
        }
    }

    let total_targets = args.target_count();
    let is_multi = args.multiattack_enabled();
    if is_multi {
        println!("  {} {}  {} {}",
            tc("⚔", COL_MED),
            tc("Multi-Attack engaged", OSAKA_JADE_B).bold(),
            tc("→", COL_DIM),
            tc(&format!("{} concurrent targets", total_targets), LAVENDER_B));
    }
    println!("  {} {}",
        tc("◈", COL_MED),
        tc("Launching scan — sit tight", LAVENDER_B).bold());
    println!();
    
    let (findings, hybrid_scanner, total_reqs): (Vec<_>, Option<_>, usize) = if is_multi {
        let per_target = (args.threads / total_targets).max(1);
        let mut scanners = Vec::new();
        for (i, target_url) in args.url.iter().enumerate() {
            let mut target_args = args.clone();
            target_args.url = vec![target_url.clone()];
            target_args.threads = per_target;
            match HybridScanner::new(target_args) {
                Ok(scanner) => {
                    scanners.push((i + 1, scanner, target_url.clone()));
                }
                Err(e) => {
                    eprintln!("  [Target {}] Failed to initialize: {}", i + 1, e);
                }
            }
        }
        let mut all_findings = Vec::new();
        let mut total_reqs = 0usize;
        let global_start = std::time::Instant::now();
        let duration_limit = if args.duration > 0 {
            Some(std::time::Duration::from_secs(args.duration))
        } else {
            None
        };
        for (idx, mut scanner, url) in scanners {
            if let Some(limit) = duration_limit {
                if global_start.elapsed() >= limit {
                    println!("  {} {} {} — global duration reached",
                        tc("⏹", COL_MED),
                        tc(&format!("[Target {}]", idx), COL_DIM),
                        tc("skipped", COL_DIM));
                    continue;
                }
            }
            match scanner.run_hybrid_scan().await {
                Ok(f) => {
                    total_reqs += scanner.req_count.load(std::sync::atomic::Ordering::Relaxed);
                    println!("  {} {}  {}",
                        tc("✓", OSAKA_JADE_B),
                        tc(&format!("[Target {}] done", idx), COL_INFO),
                        tc(&format!("{} findings", f.len()), OSAKA_JADE_B));
                    all_findings.extend(f);
                }
                Err(e) => {
                    eprintln!("  [Target {}] Scan failed: {} ({})", idx, e, url);
                }
            }
        }
        let summary = format!("Multi-Attack complete — {} total findings across {} targets",
            all_findings.len(), total_targets);
        println!("  {} {}", tc("⚔", COL_MED), tc(&summary, LAVENDER_B));
        (all_findings, None, total_reqs)
    } else if args.get_modules().contains(&"engine".to_string()) {
        println!("Using legacy ScanEngine...");
        let engine = match ScanEngine::new(args.clone()) {
            Ok(e) => e,
            Err(e) => {
                eprintln!("{} Failed to create HTTP client: {}", "[ERROR]".red().bold(), e);
                process::exit(1);
            }
        };
        match engine.run().await {
            Ok(_) => (Vec::new(), None, 0),
            Err(e) => {
                eprintln!("{} ScanEngine failed: {}", "[ERROR]".red().bold(), e);
                process::exit(1);
            }
        }
    } else {
        let mut hybrid_scanner = match HybridScanner::new(args.clone()) {
            Ok(scanner) => scanner,
            Err(e) => {
                eprintln!("{} Failed to initialize scanner: {}", "[ERROR]".red().bold(), e);
                process::exit(1);
            }
        };
        
        match hybrid_scanner.run_hybrid_scan().await {
            Ok(f) => {
                println!("[+] Scan complete! Time to review the carnage...");
                (f, Some(hybrid_scanner), 0)
            }
            Err(e) => {
                eprintln!("\n{} Scan failed: {}", "[FAILED]".red().bold(), e);
                process::exit(1);
            }
        }
    };
    
    let elapsed = start_time.elapsed();
    let req_count = if total_reqs > 0 {
        total_reqs
    } else {
        hybrid_scanner.as_ref()
            .map(|s| s.req_count.load(std::sync::atomic::Ordering::Relaxed))
            .unwrap_or(0)
    };

    Output::print_scan_complete(
        &format!("{:.1}s", elapsed.as_secs_f64()),
        req_count,
        &findings,
    );
    
    if findings.is_empty() {
        println!("  {} {}", tc("◈", OSAKA_JADE_B), tc("No vulnerabilities found — target appears secure", LAVENDER_BLUE));
    } else {
        println!("  {} {}", tc("◈", COL_MED), tc(&format!("Found {} issue(s):", findings.len()), LAVENDER_BLUE));
        for f in &findings {
            Output::print_finding_stylish(
                &format!("{:?}", f.severity),
                &f.title,
                &f.url,
                &f.evidence,
            );
        }
    }

    if let Some(scanner) = &hybrid_scanner {
        let detailed_findings = scanner.get_findings();
        if !detailed_findings.is_empty() && args.verbose {
            println!("  {}", tc("Detailed findings:", COL_DIM).underline());
            for (idx, finding) in detailed_findings.iter().take(10).enumerate() {
                println!("    {}. {} — {}",
                    tc(&format!("{:>2}", idx + 1), COL_DIM),
                    tc(&finding.title, LAVENDER_B),
                    tc(&finding.url[..finding.url.len().min(60)], LAVENDER_BLUE));
            }
            if detailed_findings.len() > 10 {
                println!("    {} {} more findings not shown", tc("⋯", COL_DIM), detailed_findings.len() - 10);
            }
        }
    }

    let final_duration = TimeUtil::format_duration(elapsed);
    println!("  {} {}    {} {}",
        tc("·", OSAKA_JADE_B), tc(&format!("Duration: {}", final_duration), LAVENDER_BLUE),
        tc("·", OSAKA_JADE_B), tc(&format!("Ended: {}", TimeUtil::format_timestamp(&TimeUtil::now())), COL_DIM));
    
    if let Some(output_path) = &args.output {
        let mut reporter = report::generator::ReportGenerator::new(&args.format);
        for finding in &findings {
            reporter.add_finding(finding.clone());
        }
        
        reporter.print_summary();
        
        let output_path = std::path::PathBuf::from(output_path);
        match reporter.save(&output_path) {
            Ok(_) => println!("\n{} Report saved to: {}", Colors::ok("[OK]"), output_path.display()),
            Err(e) => eprintln!("\n{} Failed to save report: {}", "[ERROR]".red(), e),
        }
    }
    
    // Use Colors::ok for final status display
    println!("{}", Colors::ok(&format!("Scan complete: {} vulnerabilities found", findings.len())));
    colors::print_status("OK", &format!("Found {} vulnerabilities", findings.len()));
    
    let farewells = vec![
        "Until next time, keep your patches tight!",
        "Scan finished. Go forth and remediate!",
        "Mission accomplished. Time for a victory lap!",
        "Done! Now go fix those bugs before they bite back!",
        "Scan complete. Don't forget to blame the intern!",
    ];
    let bye_idx = (TimeUtil::unix_timestamp() as usize) % farewells.len();
    println!("[+] {}", farewells[bye_idx].bright_green());
    
    // Use additional TimeUtil functions
    let utc_now = TimeUtil::now_utc();
    println!("Scan completed at (UTC): {}", TimeUtil::format_timestamp_iso(&utc_now));
    
    // Use TimeUtil::elapsed_since with a new instant
    let test_start = std::time::Instant::now();
    TimeUtil::sleep(std::time::Duration::from_millis(10));
    let _test_elapsed = TimeUtil::elapsed_since(test_start);
    
    // Use sleep_async and timeout
    let sleep_future = TimeUtil::sleep_async(std::time::Duration::from_millis(10));
    let _ = TimeUtil::timeout(std::time::Duration::from_millis(100), sleep_future).await;
    
    println!("\n{} Scan completed successfully", "[DONE]".green().bold());
}
