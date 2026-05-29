**OXIDE** (Open eXtensible Intelligence & Detection Engine) is a high-performance, AI-augmented web vulnerability scanner written entirely in **Rust**. It combines raw systems-level performance with machine learning-driven detection to find vulnerabilities that traditional scanners miss.

<div align="center">

```
   ____ _  __ ________  ______
  / __ \ |/ //  _/ __ \/ ____/
 / / / /   / // / / / / __/
/ / / /   |_/ // /_/ / /___
\____/_/|_/___/_____/_____/
```

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-00b478?style=for-the-badge&logo=rust&logoColor=c9d1c3)](https://rustup.rs/)
[![Version](https://img.shields.io/badge/version-8.5.0-00b478?style=for-the-badge)](https://github.com/hypersecuritylabs/oxide-communityedition-v8.5.0)
[![License](https://img.shields.io/badge/license-Proprietary-c4a7e7?style=for-the-badge)](LICENSE)
[![Platform](https://img.shields.io/badge/Linux%20%7C%20Windows-00c8ff?style=for-the-badge)]()
[![Kali](https://img.shields.io/badge/Kali_Linux-⭐_Star_for_Package-00b478?style=for-the-badge&logo=kalilinux&logoColor=c9d1c3)](https://github.com/hypersecuritylabs/oxide-communityedition-v8.5.0)
[![Async](https://img.shields.io/badge/async-Tokio-00c8ff?style=for-the-badge&logo=rust&logoColor=c9d1c3)](https://tokio.rs)


### **The Most Powerful AI-Augmented Web Vulnerability Scanner — Built with Rust**
#### *⭐ Star us on GitHub to help bring OXIDE to Kali Linux packages!*
#### *Built with 🦀 Rust · Powered by AI/ML · Forged in the Offensive Security Trenches*

---

> **⚠️ LEGAL WARNING & COPYRIGHT NOTICE**
>
> This tool is developed and maintained by **[khaninkali](https://github.com/hypersecuritylabs)** @ **[HyperSecurityLabs](https://hypersecuritylabs.netlify.app)**.
> Unauthorized copying, redistribution, or use of this codebase — in whole or in part — without
> explicit written permission is **strictly prohibited** and may result in legal action.
>
> OXIDE is intended **exclusively** for authorized penetration testing, security research,
> and educational purposes. **You are solely responsible** for ensuring you have proper
> authorization before scanning any target. Misuse of this tool against systems you do not
> own or have explicit permission to test is **illegal** and unethical.
>
> © 2024-2025 khaninkali · HyperSecurityLabs · All Rights Reserved

</div>

---

## 🧬 What is OXIDE?

**OXIDE** (Open eXtensible Intelligence & Detection Engine) is the **most powerful AI-augmented web vulnerability scanner** written entirely in **Rust**. Unlike traditional scanners that rely on signatures alone, OXIDE combines raw systems-level performance with machine learning-driven detection to find vulnerabilities that other tools miss.

From classic SQLi and XSS to zero-day anomaly detection using real ML models — OXIDE is built for the modern offensive security professional and deserves a spot at the top of every pentester's toolkit. **Star us on GitHub** to help bring OXIDE to Kali Linux as an official package!

---

## ⚡ Feature Highlights

### 🔍 Vulnerability Scanners
| Module | Description |
|--------|-------------|
| `sqli` | SQL Injection — error-based, blind, time-based |
| `blind-sqli` | Blind SQLi with timing analysis |
| `xss` | Cross-Site Scripting — reflected, stored, DOM |
| `lfi` | Local File Inclusion with path traversal chains |
| `path-traversal` | Directory traversal across OS variants |
| `cmd-injection` | OS Command Injection detection |
| `cors` | CORS misconfiguration assessment |
| `tls` | Full TLS/SSL security audit |
| `creds` | Default credential brute-force (6000+ combos) |
| `common` | Nikto-style common app checks (2790+ tests) |
| `db-fingerprint` | Database engine fingerprinting |
| `insta` | Instagram OSINT — follower count, private status, profile pic |
| `session` | Session hijack testing — cookie flags, fixation, predictability |
| `train` | ML classifier trainer — learns from live scanner results |

### 🤖 AI / ML Engine
- **Zero-Day Detection** — statistical anomaly detection using `smartcore` (Random Forest, SVM) and `linfa` clustering
- **Pattern Learner** — adaptive payload mutation based on response patterns
- **Exploit Analyzer** — AI-driven exploit chain analysis
- **Response Analyzer** — behavioral fingerprinting of HTTP responses
- **Payload Mutator** — ML-guided payload evolution

### 🕷️ Crawling & Discovery
- Async multi-threaded web crawler with configurable depth and URL limits
- JavaScript-aware crawling (`crawler_js`)
- Automatic parameter discovery across all crawled URLs
- Form input extraction and endpoint mapping

### 🛡️ v8.5.0 Improvements

| Improvement | Details |
|-------------|---------|
| **Duration Enforcement** | Global timer + per-payload deadline checks; ±1s accuracy |
| **Request Counting Fixed** | SCAN COMPLETE shows actual HTTP request count |
| **Color Audit** | Full Osaka-Jade & Lavender palette across all UI components |
| **Findings Display** | All findings printed unconditionally after scan (no verbose gate) |
| **SCAN COMPLETE Redesign** | ANSI-aware `vis()` padding, `─` borders with `│` corners |
| **Severity Badges** | `[CRIT]` / `[HIGH]` / `[MEDIUM]` / `[LOW]` / `[INFO]` format |
| **Code Cleanup** | Zero warnings, unused imports removed |

### 📊 Reporting
- Output formats: **JSON**, **HTML**, **CSV**, **XML**
- Severity-classified findings: Critical / High / Medium / Low / Info
- Auto-download of sensitive discovered files (`--download`)
- Verbose mode with full evidence and remediation guidance

---

## ⭐ Help Bring OXIDE to Kali Linux

OXIDE aims to become an **official Kali Linux package**. Star us on [GitHub](https://github.com/hypersecuritylabs/oxide-communityedition-v8.5.0) to show your support — the more stars we get, the higher the priority for Kali inclusion.

```bash
# Until then, build from source:
git clone https://github.com/hypersecuritylabs/oxide-communityedition-v8.5.0
cd oxide-communityedition-v8.5.0
cargo build --release
./target/release/oxide --url https://target.com --modules all
```

**[⭐ Star on GitHub](https://github.com/hypersecuritylabs/oxide-communityedition-v8.5.0) → Kali package soon.**

## 🚀 Installation

### Prerequisites
- [Rust](https://rustup.rs/) 1.75+ (2021 edition)
- Cargo (bundled with Rust)
- **Linux** (primary target) / **Windows** / **macOS**

### Build from Source

```bash
git clone https://github.com/hypersecuritylabs/oxide-communityedition-v8.5.0
cd oxide-communityedition-v8.5.0

# Quick build (default debug)
cargo build

# Release build
cargo build --release

# Release build with Evergreens theme
./oxide_build.sh --release
```

The binary will be at `./target/release/oxide` or `./target/debug/oxide`.

### ⚙️ Proxy Shared Library

OXIDE requires a proxy dynamic library — `liboxide_proxy.so` (Linux) or `liboxide_proxy.dll` (Windows) — that provides proxy routing, authentication, URL obfuscation, and rotation logic. The binary **refuses to run** without it.

**Build it:**
```bash
cd oxide-proxy
cargo build --release
```

**Install it** (one of):
```bash
# Next to the binary (auto-detected)
cp oxide-proxy/target/release/liboxide_proxy.so target/release/

# System-wide (Linux)
sudo cp oxide-proxy/target/release/liboxide_proxy.so /usr/lib/

# Custom path (Linux)
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/path/to/liboxide_proxy.so
```

On **Windows**, the file is `liboxide_proxy.dll`; place it in the same directory as `oxide.exe` or set `PATH`.

### Build Script

Use `oxide_build.sh` for optimized builds:

```bash
# Default debug build with Evergreens theme
./oxide_build.sh

# Release build with Evergreens theme
./oxide_build.sh --release
```

### Optimized Release Build

The release profile is pre-configured for maximum performance:
```toml
opt-level = 3
lto = "thin"
codegen-units = 1
```

---

## 🪟 Windows Compatibility

OXIDE is built with pure Rust and **cross-compiles to Windows** via both `x86_64-pc-windows-gnu` (MinGW) and `x86_64-pc-windows-msvc` (MSVC).

| Component | Windows Status |
|-----------|---------------|
| Core scanner | ✅ Full support |
| CLI & display | ✅ Full support (colored output) |
| SQLite DB | ✅ `rusqlite` with bundled SQLite |
| HTTP client | ✅ reqwest with native-tls or rustls |
| Proxy library | ✅ Builds as `liboxide_proxy.dll` (cdylib) |
| Instagram OSINT | ✅ Pure HTTP requests |
| Session hijack | ✅ Pure HTTP requests |
| ML engine | ✅ smartcore / linfa / ndarray |
| Rate limiter | ✅ governor |
| DNS resolver | ✅ trust-dns-resolver |
| `pnet` (raw packet) | ⚠️ Limited — disable with `--no-default-features` |

**Build for Windows (cross-compile from Linux):**
```bash
# GNU toolchain (recommended — no MSVC linker required)
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu

# MSVC toolchain (requires mingw-w64 or native MSVC)
rustup target add x86_64-pc-windows-msvc
cargo build --release --target x86_64-pc-windows-msvc
```

**Or build natively on Windows:**
```powershell
cargo build --release
# Place liboxide_proxy.dll next to oxide.exe
```

**Environment variable** (Windows): `set OXIDE_DB_DIR=C:\path\to\database`

---

## 🧩 Proxy Library System

### How It Works

The proxy system is split into two parts:

**1. `oxide-proxy/` (cdylib → `liboxide_proxy.so`)**

A standalone Rust library compiled as a C-compatible dynamic library (`cdylib`) exporting 7 `extern "C"` functions:

| Export | Signature | Purpose |
|--------|-----------|---------|
| `proxy_ping` | `() -> u32` | Health check — returns version string length |
| `proxy_route` | `(target, *mut Config) -> i32` | Selects proxy type (HTTP/HTTPS/SOCKS5) based on target URL |
| `proxy_auth` | `(username, password) -> i32` | Validates and Base64-encodes proxy credentials |
| `proxy_obfuscate` | `(input, *mut buf, max_len) -> i32` | XOR (0xAA) obfuscates proxy URLs |
| `proxy_deobfuscate` | `(input, *mut buf, max_len) -> i32` | Reverses obfuscation |
| `proxy_rotation_seed` | `() -> u64` | Time-based seed for proxy rotation pool |
| `proxy_validate` | `(*const Config) -> i32` | Validates proxy configuration struct (host, port) |

The .so is built with `opt-level = "z"`, `lto = "fat"`, `panic = "abort"`, and stripped — resulting in a **278KB** binary footprint.

**2. `src/http/proxy_loader.rs` (the loader)**

Loaded at startup via `libloading`. The flow:

```
main() → ensure_proxy_library()
           │
           ├─ Finds .so (CWD → /usr/lib/ → /usr/local/lib/ → /opt/oxide/lib/ → LD_LIBRARY_PATH)
           ├─ Opens via libloading::Library
           ├─ Verifies all 7 symbols exist
           ├─ Stores Arc<Library> in OnceLock global
           └─ Prints version on success, exits on failure
```

Once loaded, safe Rust wrappers (`proxy_route()`, `proxy_auth()`, etc.) call the C ABI functions with proper `CString` marshalling. The global `OnceLock` ensures the library is loaded **once** at startup — subsequent calls are lock-free reads.

### Why a Shared Library?

- **Runtime enforcement** — binary refuses to run without it; prevents standalone binary abuse
- **Updatable independently** — replace the .so without recompiling the scanner
- **FFI sandbox** — proxy logic runs in a separate compilation unit with `panic=abort`
- **Obfuscation isolation** — XOR keys live in the .so, not in the main binary

---

## 🎯 Usage

```bash
oxide --url <TARGET> [OPTIONS]
```

### Basic Scan

```bash
oxide --url https://target.example.com
```

### Full Scan with All Modules

```bash
oxide --url https://target.example.com --modules all --threads 20 --verbose
```

### Targeted Module Scan

```bash
oxide --url https://target.example.com --modules sqli,xss,cors,tls
```

### Save Report

```bash
oxide --url https://target.example.com --output report.html --format html
```

### Zero-Day Detection Mode

```bash
oxide --url https://target.example.com --zeroday --verbose
```

### Stealth / Rate-Limited Scan

```bash
oxide --url https://target.example.com --rate-limit 5 --silent-mode
```

### With Authentication

```bash
oxide --url https://target.example.com \
  --cookie "session=abc123" \
  --header "Authorization: Bearer <token>"
```

---

## 🧰 CLI Reference

| Flag | Default | Description |
|------|---------|-------------|
| `-u, --url` | required | Target URL |
| `-t, --threads` | `20` | Concurrent worker threads |
| `-o, --output` | — | Output file path |
| `-f, --format` | `json` | Report format: `json`, `html`, `csv`, `xml` |
| `--modules` | `all` | Comma-separated module list |
| `--exclude` | — | Modules to skip |
| `--exploitation-level` | `50` | Aggression level (1–100) |
| `--payload-limit` | `50` | Max payloads per endpoint |
| `--crawl-depth` | `3` | Spider recursion depth |
| `--max-urls` | `100` | Max URLs to crawl |
| `--rate-limit` | `0` (unlimited) | Requests per second cap |
| `--proxy` | — | HTTP/HTTPS proxy URL |
| `--user-agent` | — | Custom User-Agent string |
| `--cookie` | — | Session cookies |
| `--header` | — | Extra request headers (repeatable) |
| `--follow-redirects` | false | Follow HTTP redirects |
| `--max-redirects` | `10` | Max redirect chain depth |
| `--duration` | `0` (unlimited) | Max scan duration in seconds with live countdown |
| `--insecure` | false | Disable TLS verification |
| `--download` | false | Auto-save discovered sensitive files |
| `--zeroday` | false | Enable zero-day ML detection |
| `--train` | false | Train ML classifier from live scanner results |
| `--insta` | false | Enable Instagram OSINT module |
| `--session` | false | Enable session hijack testing module |
| `--silent-mode` | false | Suppress non-essential output |
| `-v, --verbose` | false | Full verbose output |

### Available Modules

```
all · engine · static · agent · body · fingerprint · tls · common
cors · creds · sqli · xss · lfi · path-traversal · cmd-injection
blind-sqli · db-fingerprint · parameter-discovery · fuzz · insta
session · zeroday · train
```

---

## 🏗️ Architecture

```
OxideCommunityEdtionv8.5.0/
├── src/
│   ├── main.rs              # Entry point & banner (Evergreens)
│   ├── hybrid.rs            # HybridScanner orchestrator (12-phase scan pipeline)
│   ├── agent.rs             # AgentPool — parallel agent-based scanning
│   ├── crawls.rs            # Async web crawler
│   ├── recon.rs             # Reconnaissance module
│   ├── db.rs                # Encrypted SQLite DB loader (XOR decrypt + rusqlite)
│   ├── filter.rs            # False-positive filter
│   ├── lib.rs               # Crate root — all `pub mod` declarations
│   │
│   ├── insta/               # Instagram OSINT
│   │   └── mod.rs           # follower_count, is_private, download_profile_pic
│   ├── session_hijack/      # Session hijack testing
│   │   └── mod.rs           # cookie flags, fixation, predictability
│   │
│   ├── core/                # Scan engine core
│   │   ├── engine.rs        # ScanEngine
│   │   ├── scanner.rs       # Scanner primitives
│   │   ├── worker.rs        # ParallelScanner + WorkerPool
│   │   ├── coordinator.rs   # Progress tracking
│   │   └── dispatcher.rs    # Parallel HTTP dispatcher
│   │
│   ├── scanner/             # Vulnerability-specific scanners
│   │   ├── sqli_scanner.rs
│   │   ├── blind_sqli_scanner.rs
│   │   ├── xss_scanner.rs
│   │   ├── lfi_scanner.rs
│   │   ├── path_traversal_scanner.rs
│   │   ├── cmd_injection_scanner.rs
│   │   ├── cors_scanner.rs
│   │   ├── tls_scanner.rs
│   │   ├── default_creds_scanner.rs
│   │   ├── common_app_scanner.rs
│   │   ├── db_fingerprinter.rs
│   │   └── precision.rs
│   │
│   ├── ai/                  # AI-powered analysis
│   │   ├── exploit_analyzer.rs
│   │   ├── response_analyzer.rs
│   │   ├── payload_mutator.rs
│   │   └── pattern_learner.rs
│   │
│   ├── zero_DAY/            # Zero-day ML detection
│   │   ├── engine.rs        # ZeroDayEngine
│   │   ├── classifier.rs    # ML classifier (Random Forest / SVM)
│   │   ├── baseline.rs      # Behavioral baseline builder
│   │   ├── anomaly.rs       # Anomaly scoring
│   │   ├── features.rs      # Feature extraction
│   │   └── trainer.rs       # ML trainer — auto-index scanners → train → export model
│   │
│   ├── advanced/            # Advanced offensive features
│   │   ├── api_fuzzer.rs
│   │   ├── evasion.rs
│   │   ├── websocket.rs
│   │   ├── cluster.rs
│   │   ├── session.rs
│   │   ├── ml_detector.rs
│   │   ├── crawler_js.rs
│   │   ├── rate_limiter.rs
│   │   ├── cache.rs
│   │   └── plugin.rs
│   │
│   ├── payload/             # Payload generation & mutation
│   │   ├── generator.rs
│   │   ├── mutator.rs
│   │   ├── fuzzer.rs
│   │   ├── encoder.rs
│   │   ├── sql_injection.rs
│   │   ├── xss.rs
│   │   ├── lfi.rs
│   │   ├── command_injection.rs
│   │   └── path_traversal.rs
│   │
│   ├── detection/           # Detection & analysis
│   │   ├── analyzer.rs      # Finding + Severity types
│   │   ├── signatures.rs    # Vulnerability signature DB
│   │   ├── behavior.rs      # Behavioral analysis
│   │   ├── timing.rs        # Time-based detection
│   │   ├── confirm.rs       # Vulnerability confirmation
│   │   └── matcher.rs
│   │
│   ├── http/                # HTTP layer
│   │   ├── client.rs        # Async HTTP client (reqwest)
│   │   ├── request.rs
│   │   ├── response.rs
│   │   ├── headers.rs
│   │   ├── cookies.rs
│   │   ├── redirect.rs
│   │   ├── tls.rs
│   │   ├── useragents.rs
│   │   └── proxy_loader.rs  # Dynamic library loader (FFI wrappers)
│   │
│   ├── report/              # Report generation
│   │   ├── generator.rs
│   │   ├── json.rs
│   │   ├── html.rs
│   │   ├── csv.rs
│   │   └── xml.rs
│   │
│   ├── cli/                 # CLI interface
│   │   ├── args.rs          # Clap argument definitions
│   │   ├── display.rs       # Terminal display engine (Evergreens)
│   │   ├── output.rs
│   │   ├── progress.rs
│   │   ├── spinner.rs
│   │   ├── colors.rs
│   │   ├── config.rs
│   │   └── parser.rs
│   │
│   └── utils/               # Utilities
│       ├── url.rs
│       ├── encoding.rs      # Base64, hex, URL, HTML, unicode
│       ├── time.rs
│       └── downloader.rs
│
└── oxide-proxy/             # Proxy shared library (cdylib)
    ├── Cargo.toml
    └── src/
        └── lib.rs           # 7 extern "C" exports

```

---

## 📋 Changelog — v8.5.0

### Duration Enforcement
- **Global duration timer** added before multiattack target loop — skips remaining targets if time exhausted
- **Per-payload deadline checks** inside `fuzz_url` — returns early when deadline passed
- **Reduced grace period** from 5s to 1s in `check_duration` — scans stop within seconds of the limit
- **Crawl phase deadline checks** — verifies time before and after crawling, returns empty URL list if expired

### Request Counting Fixed
- Multiattack mode: `total_reqs` accumulates `scanner.req_count` per target
- `hybrid_scanner.req_count` (AtomicUsize) tracks every live HTTP request
- SCAN COMPLETE now displays **actual HTTP request count** instead of `findings.len()`
- Early returns in `run_hybrid_scan` propagate accumulated request count instead of 0

### Full Color Audit — Osaka-Jade & Lavender Palette
- `render_scan_bar` / `server_badge` / `WorkerSlot` / `AgentBar` / `Colors` / `sev_badge` / `print_finding` — all updated to jade `(0,180,120)` / bright jade `(80,240,180)` / lavender `(196,167,231)`
- `Colors::warning` → light cyan `(0,200,255)`, `Colors::brand` → matrix green `(0,220,80)`
- Severity badges: `▌CRITICAL▐` → `[CRIT]`, low severity summary GB_AQU → GB_GRN (jade)
- URLs in findings: `GB_BLU_B` → light cyan `(0,200,255)`
- SCAN COMPLETE: jade `─` borders with `│` corners, ANSI-aware `vis()` padding
- Multiattack text: `GB_RED_B` → `OSAKA_JADE_B`

### Findings Display Fixed
- **Always print findings** after scan — removed `args.verbose` gate and `findings.len() < 5` shortcut
- SCAN COMPLETE redesigned with proper ANSI-aware right-alignment

### Code Cleanup
- Unused imports removed: `GB_RED_B` from `main.rs`
- Zero compiler warnings on every build

### Pro Edition (v9.1.0) Improvements
- Welcome screen: dividers/title → jade, capabilities → bright lavender/cyan, INIT bar → jade gradient
- Target Information block: `─` borders with `│` corners, lavender title
- Verbose mode: section headers per test type (`+ --- /SQLi ------ > url`) with per-payload GET lines
- Non-verbose fuzzing header: STATUS/METHOD/LINES/WORDS/CHARS/TYPE columns in jade
- Ai-Powered FUZZING title: jade `(0,180,120)`
- SCAN COMPLETE: jade `─` borders, `│` corners, `[CRIT]`/`[HIGH]`/`[MEDIUM]`/`[LOW]`/`[INFO]` badges

---

## 🔬 Tech Stack

| Crate | Purpose |
|-------|---------|
| `tokio` | Async runtime |
| `reqwest` | HTTP client (gzip, brotli, cookies) |
| `clap` | CLI argument parsing |
| `scraper` | HTML parsing & crawling |
| `smartcore` | ML algorithms (Random Forest, SVM) |
| `linfa` | ML clustering & preprocessing |
| `ndarray` | N-dimensional arrays for ML |
| `statrs` | Statistical distributions |
| `rustls` | TLS implementation |
| `trust-dns-resolver` | DNS resolution |
| `governor` | Rate limiting |
| `libloading` | Dynamic library loading (proxy .so, plugins) |
| `base64` | Proxy credential encoding |
| `serde` / `serde_json` | Serialization |
| `colored` | Terminal color output |
| `regex` | Pattern matching |
| `sha2` / `sha1` / `md5` | Cryptographic hashing |
| `uuid` | Unique scan identifiers |
| `chrono` | Timestamps & duration |
| `rusqlite` | Embedded SQLite database engine (bundled) |
| `csv` | CSV parsing (legacy fallback) |

---

## 👤 Author

<div align="center">

**[khaninkali](https://github.com/hypersecuritylabs)** · *[HyperSecurityLabs](https://hypersecuritylabs.netlify.app)*

[![GitHub](https://img.shields.io/badge/GitHub-@hypersecuritylabs-00b478?style=for-the-badge&logo=github&logoColor=c9d1c3)](https://github.com/hypersecuritylabs)
[![Website](https://img.shields.io/badge/Website-hypersecuritylabs.netlify.app-00c8ff?style=for-the-badge&logo=google-chrome&logoColor=c9d1c3)](https://hypersecuritylabs.netlify.app)
[![Telegram](https://img.shields.io/badge/Telegram-@hypersecurity__offsec-c4a7e7?style=for-the-badge&logo=telegram&logoColor=c9d1c3)](https://t.me/hypersecurity_offsec)

*"Scan everything. Trust nothing. Patch accordingly."*

</div>

---

## ⚖️ License & Legal

**Proprietary Software License** — Copyright © 2024-2025 khaninkali · HyperSecurityLabs · All Rights Reserved

See [`LICENSE`](LICENSE) for full terms. In short:

| Action | Public | HyperSecurity Members |
|--------|--------|----------------------|
| View source | ✅ Yes | ✅ Yes |
| Fork for reference | ✅ Yes | ✅ Yes |
| Personal / educational use | ✅ Yes | ✅ Yes |
| Authorized pentesting | ✅ Yes | ✅ Yes |
| Modify code | ❌ No | ✅ Yes |
| Submit PRs / merge | ❌ No (may submit, no merge rights) | ✅ Yes |
| Remove name / tags / attribution | ❌ No — legal action | ❌ No — never |
| Rebrand as own work | ❌ No — legal action | ❌ No — never |
| Sell / monetize | ❌ No — written permission only | ❌ No — written permission only |
| Redistribute | ❌ No — written permission only | ✅ Yes (with attribution) |

**Only HyperSecurityLabs members may modify, merge, or redistribute this code. Removing the author name, version tags, or HyperSecurityLabs branding is a direct license violation and will result in legal action.**

> **By using OXIDE, you agree that you have obtained proper authorization for all targets
> and that you bear full legal and ethical responsibility for your actions.**

---

<div align="center">

*Built with 🦀 Rust · Forged in the offensive security trenches*

**[HyperSecurityLabs](https://hypersecuritylabs.netlify.app)** · OXIDE Framework v8.5.0

[🐙 GitHub](https://github.com/hypersecuritylabs) · [🌐 Website](https://hypersecuritylabs.netlify.app) · [💬 Telegram](https://t.me/hypersecurity_offsec)

</div>
