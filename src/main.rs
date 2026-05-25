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
    EV_AQU, EV_AQU_B, EV_BLU_B, EV_FG, EV_FG0, EV_GRN_B, EV_GRY, EV_RED_B, EV_YLW, EV_YLW_B,
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
        EV_AQU, EV_FG0, EV_GRN_B, EV_GRY, EV_YLW_B, EV_GRN,
    };
    println!();
    println!("{}", tc("   ____ _  __ ________  ______", EV_GRN_B));
    println!("{}", tc("  / __ \\ |/ //  _/ __ \\/ ____/", EV_YLW_B));
    println!("{}", tc(" / / / /   / / // / / / __/", EV_YLW_B));
    println!("{}", tc("/ /_/ /   |_/ // /_/ / /___", EV_AQU));
    println!("{}", tc("\\____/_/|_/___/_____/_____/", EV_GRY));
    println!();
    println!("{}", tc("═══════════════════════════════════════════════════════════════════════════════", EV_YLW_B));
    println!("  {} {}",
        tc("◈", EV_YLW_B),
        tc("Hypersecurity Offensive Labs  |  OXIDE Community Edition v8.4.9", EV_FG0));
    println!("  {} {}",
        tc(">>", EV_AQU),
        tc("Open eXtensible Intelligence & Detection Engine — Community Edition << /evergreen okay", EV_GRY));
    println!("{}", tc("═══════════════════════════════════════════════════════════════════════════════", EV_GRN));
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
        EV_AQU, EV_BLU_B, EV_FG, EV_FG0, EV_GRN_B, EV_GRY, EV_RED_B, EV_YLW_B,
    };

    Output::print_header("Target Information");
    if args.multiattack_enabled() {
        println!("  {} {}  {} {}",
            tc("▸", EV_YLW_B), tc("Multi-Attack", EV_GRY).bold(),
            tc("→", EV_GRY), tc(&format!("{} targets", args.target_count()), EV_RED_B));
        let per_target = (args.threads / args.target_count()).max(1);
        for (i, url) in args.url.iter().enumerate() {
            let clean = Parser::ensure_http(url);
            let _host = url::Url::parse(&clean)
                .ok()
                .and_then(|u| u.host_str().map(|h| h.to_string()))
                .unwrap_or_default();
            println!("  {} {}  {} {}  {} {}",
                tc("▸", EV_YLW_B), tc(&format!("Target {}", i + 1), EV_GRY).bold(),
                tc("→", EV_GRY), tc(&clean, EV_FG0),
                tc("≈", EV_YLW_B), tc(&format!("{} thr", per_target), EV_GRN_B));
        }
        println!("  {} {}  {} {}  {} {}",
            tc("▸", EV_YLW_B), tc("Threads", EV_GRY).bold(),
            tc("→", EV_GRY), tc(&format!("{} total", args.threads), EV_GRN_B),
            tc("⏱", EV_YLW_B), tc(&format!("{}s duration", args.duration), EV_FG));
    } else {
        let clean = Parser::ensure_http(args.target_url());
        let host = url::Url::parse(&clean)
            .ok()
            .and_then(|u| u.host_str().map(|h| h.to_string()))
            .unwrap_or_default();
        let ips = resolve_ip(&host).await;
        let ip_display = if ips.is_empty() {
            tc("unresolved", EV_RED_B)
        } else {
            tc(&ips.join(", "), EV_GRN_B)
        };
        println!("  {} {}  {} {}",
            tc("▸", EV_YLW_B), tc("Target", EV_GRY).bold(),
            tc("→", EV_GRY), args.target_url().truecolor(EV_FG0.0, EV_FG0.1, EV_FG0.2).bold());
        println!("  {} {}  {} {}",
            tc("▸", EV_YLW_B), tc("IP", EV_GRY).bold(),
            tc("→", EV_GRY), ip_display);
        println!("  {} {}  {} {}  {} {}",
            tc("▸", EV_YLW_B), tc("Threads", EV_GRY).bold(),
            tc("→", EV_GRY), tc(&args.threads.to_string(), EV_GRN_B),
            tc("⏱", EV_YLW_B), tc(&format!("{}s duration", args.duration), EV_FG));
    }

    let modules = args.get_modules();
    let module_line: Vec<String> = modules.iter().map(|m| tc(m, EV_AQU)).collect();
    println!("  {} {}  {} {}",
        tc("▸", EV_YLW_B), tc("Modules", EV_GRY).bold(),
        tc("→", EV_GRY), module_line.join(tc(" │ ", EV_GRY).as_str()));

    if let Some(output) = &args.output {
        println!("  {} {}  {} {}",
            tc("▸", EV_YLW_B), tc("Output", EV_GRY).bold(),
            tc("→", EV_GRY), tc(output, EV_GRN_B));
    }

    if args.verbose  { println!("  {} {}", tc("▸", EV_YLW_B), tc("Verbose mode", EV_GRN_B)); }
    if args.insecure { println!("  {} {}", tc("▸", EV_YLW_B), tc("SSL verification disabled", EV_RED_B)); }
    if args.zeroday  { println!("  {} {}", tc("▸", EV_YLW_B), tc("Zero-day detection", EV_YLW_B)); }
    if args.train    { println!("  {} {}", tc("▸", EV_YLW_B), tc("Training mode", EV_RED_B)); }
    if args.insta    { println!("  {} {}", tc("▸", EV_YLW_B), tc("Instagram OSINT", EV_BLU_B)); }
    if args.session  { println!("  {} {}", tc("▸", EV_YLW_B), tc("Session hijack testing", EV_AQU)); }

    if !args.header.is_empty() {
        Output::print_section("Custom Headers");
        for header in &args.header {
            match Parser::parse_header(header) {
                Ok((key, value)) => println!("    {}: {}",
                    tc(&key, EV_GRN_B), tc(&value, EV_FG0)),
                Err(e) => println!("    Invalid header '{}': {}", header, e),
            }
        }
    }

    if let Some(cookie) = &args.cookie {
        Output::print_section("Cookies");
        for (key, value) in &Parser::parse_cookie(cookie) {
            println!("    {}: {}",
                tc(&key, EV_GRN_B), tc(&value, EV_FG0));
        }
    }

    Output::print_line();
    println!();
}

#[tokio::main]
async fn main() {
    let args = match CliArgs::parse_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{} {}", "[ERROR]".red().bold(), e);
            process::exit(1);
        }
    };
    
    print_banner();
    
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
            tc("⚔", EV_YLW_B),
            tc("Multi-Attack engaged", EV_RED_B).bold(),
            tc("→", EV_GRY),
            tc(&format!("{} concurrent targets", total_targets), EV_FG0));
    }
    println!("  {} {}",
        tc("◈", EV_YLW_B),
        tc("Launching scan — sit tight", EV_FG0).bold());
    println!();
    
    let (findings, hybrid_scanner): (Vec<_>, _) = if is_multi {
        let per_target = (args.threads / total_targets).max(1);
        let mut scanners = Vec::new();
        for (i, target_url) in args.url.iter().enumerate() {
            let mut target_args = args.clone();
            target_args.url = vec![target_url.clone()];
            target_args.threads = per_target;
            match HybridScanner::new(target_args) {
                Ok(scanner) => {
                    println!("  {} {}  {}",
                        tc("▶", EV_GRN_B),
                        tc(&format!("[Target {}]", i + 1), EV_YLW_B),
                        tc(target_url, EV_FG0));
                    scanners.push((i + 1, scanner, target_url.clone()));
                }
                Err(e) => {
                    eprintln!("  [Target {}] Failed to initialize: {}", i + 1, e);
                }
            }
        }
        let mut all_findings = Vec::new();
        for (idx, mut scanner, target_url) in scanners {
            match scanner.run_hybrid_scan().await {
                Ok(f) => {
                    println!("  {} {}  {}",
                        tc("✓", EV_GRN_B),
                        tc(&format!("[Target {}] done", idx), EV_AQU),
                        tc(&format!("{} findings", f.len()), EV_GRN_B));
                    all_findings.extend(f);
                }
                Err(e) => {
                    eprintln!("  [Target {}] Scan failed: {} ({})", idx, e, target_url);
                }
            }
        }
        let summary = format!("Multi-Attack complete — {} total findings across {} targets",
            all_findings.len(), total_targets);
        println!("  {} {}", tc("⚔", EV_YLW_B), tc(&summary, EV_FG0));
        (all_findings, None)
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
            Ok(_) => (Vec::new(), None),
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
                (f, Some(hybrid_scanner))
            }
            Err(e) => {
                eprintln!("\n{} Scan failed: {}", "[FAILED]".red().bold(), e);
                process::exit(1);
            }
        }
    };
    
    let elapsed = start_time.elapsed();

    Output::print_scan_complete(
        &format!("{:.1}s", elapsed.as_secs_f64()),
        findings.len(),
        &findings,
    );
    
    if findings.is_empty() {
        println!("  {} {}", tc("◈", EV_AQU_B), tc("No vulnerabilities found — target appears secure", EV_FG));
    } else if findings.len() < 5 {
        println!("  {} {}", tc("◈", EV_YLW), tc("Found a few issues", EV_FG));
    } else {
        println!("  {} {}", tc("◈", EV_RED_B), tc(&format!("Found {} issues — review recommended", findings.len()), EV_FG));
    }

    if let Some(scanner) = &hybrid_scanner {
        let detailed_findings = scanner.get_findings();
        if !detailed_findings.is_empty() && args.verbose {
            println!("  {}", tc("Detailed findings:", EV_GRY).underline());
            for (idx, finding) in detailed_findings.iter().take(10).enumerate() {
                println!("    {}. {} — {}",
                    tc(&format!("{:>2}", idx + 1), EV_GRY),
                    tc(&finding.title, EV_FG0),
                    tc(&finding.url[..finding.url.len().min(60)], EV_BLU_B));
            }
            if detailed_findings.len() > 10 {
                println!("    {} {} more findings not shown", tc("⋯", EV_GRY), detailed_findings.len() - 10);
            }
        }
    }

    let final_duration = TimeUtil::format_duration(elapsed);
    println!("  {} {}    {} {}",
        tc("⏱", EV_YLW_B), tc(&format!("Duration: {}", final_duration), EV_FG),
        tc("◷", EV_GRN_B), tc(&format!("Ended: {}", TimeUtil::format_timestamp(&TimeUtil::now())), EV_GRY));
    
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
