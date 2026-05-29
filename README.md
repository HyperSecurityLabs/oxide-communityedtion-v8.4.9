**OXIDE** (Open eXtensible Intelligence & Detection Engine) is a high-performance, AI-augmented web vulnerability scanner written entirely in **Rust**. It combines raw systems-level performance with machine learning-driven detection to find vulnerabilities that traditional scanners miss.

<p align="center">
  <img src="https://img.shields.io/badge/version-8.5.0-50dca0?style=for-the-badge&labelColor=1a1a2e" />
  <img src="https://img.shields.io/badge/status-FINAL%20RELEASE-80dca0?style=for-the-badge&labelColor=1a1a2e" />
  <img src="https://img.shields.io/badge/license-Proprietary-beb0eb?style=for-the-badge&labelColor=1a1a2e" />
  <img src="https://img.shields.io/badge/platform-Linux%20%7C%20Windows-aac3eb?style=for-the-badge&labelColor=1a1a2e" />
  <img src="https://img.shields.io/badge/Rust-2021-edition-50dca0?style=for-the-badge&labelColor=1a1a2e&logo=rust" />
  <img src="https://img.shields.io/badge/Kali_Linux-557C94?style=for-the-badge&logo=kali-linux&logoColor=white&labelColor=1a1a2e" />
</p>

---

<h1 align="center">
  <code style="color:#50dca0;background:#1a1a2e;padding:4px 16px;border-radius:6px;border:1px solid #50dca066;">[ 🦀 OXIDE Framework v8.5.0 ]</code>
</h1>

<h3 align="center" style="color:#beb0eb;">
  Open eXtensible Intelligence & Detection Engine
</h3>

<h4 align="center">
  <em style="color:#aac3eb;">Community Edition — The Greatest Update · Final Release</em>
</h4>

<p align="center">
  <strong style="color:#50dca0;">
    Built with Rust · Powered by AI/ML · Engineered for Offensive Security
  </strong>
</p>

<br/>

<p align="center">
  <code style="color:#50dca0;font-size:1.2em;">
    ▷ This is the last freely-available Community Edition. ◁
  </code>
  <br/>
  <span style="color:#7890a8;">
    Future development moves exclusively to OXIDE Pro Edition.
  </span>
</p>

<br/>

---

<p align="center">
  <a href="https://github.com/hypersecuritylabs/oxide-communityedition-v8.5.0">
    <img src="https://img.shields.io/badge/%E2%AD%90%20Star%20us%20on%20GitHub-50dca0?style=for-the-badge&labelColor=1a1a2e" alt="Star us on GitHub" />
  </a>
  &nbsp;&nbsp;
  <a href="https://www.kali.org/tools/">
    <img src="https://img.shields.io/badge/Proudly%20crafted%20for-Kali%20Linux-557C94?style=for-the-badge&labelColor=1a1a2e&logo=kali-linux" alt="Kali Linux" />
  </a>
</p>

<p align="center" style="color:#788298;">
  <strong style="color:#50dca0;">⭐ Star this repository</strong> to support open-source security research.
  <br/>
  <span style="color:#aac3eb;">OXIDE is battle-tested and tuned for <strong style="color:#50dca0;">Kali Linux</strong> — the pentester's distro of choice.</span>
</p>

<br/>

---

<h2 style="color:#50dca0;border-bottom:1px solid #50dca066;">
  <code>[ TOC ]</code>
</h2>

- [Overview](#-overview)
- [What's New in v8.5.0](#-whats-new-in-v850-greatest-update)
- [Vulnerability Scanners](#-vulnerability-scanners)
- [AI / ML Engine](#-ai--ml-engine)
- [Architecture](#-architecture)
- [Quick Start](#-quick-start)
- [CLI Reference](#-cli-reference)
- [Display & Style System](#-display--style-system)
- [Hypersecurity Kernel Module](#-hypersecurity-kernel-module)
- [Security Hardening](#-security-hardening)
- [Code Quality](#-code-quality)
- [Distribution Packages](#-distribution-packages)
- [Kali Linux](#-kali-linux)
- [Star This Project](#-star-this-project)
- [Known Limitations](#-known-limitations)
- [License](#-license)
- [Connect](#-connect)

---

<h2 style="color:#50dca0;border-bottom:1px solid #50dca066;">
  <code>[ OVERVIEW ]</code>
</h2>

**OXIDE** is a next-generation, AI-augmented web vulnerability scanner written entirely in **Rust**. It combines systems-level performance with machine learning-driven detection to find what traditional scanners miss.

From classic SQLi and XSS to zero-day anomaly detection using real Random Forest and SVM models — OXIDE is built for the modern offensive security professional.

> **Release Date:** May 29, 2026
> **Author:** [khaninkali](https://github.com/hypersecuritylabs) · HyperSecurityLabs
> **Repository:** [github.com/hypersecuritylabs/oxide-communityedition-v8.5.0](https://github.com/hypersecuritylabs/oxide-communityedition-v8.5.0)

---

<h2 style="color:#50dca0;border-bottom:1px solid #50dca066;">
  <code>[ WHAT'S NEW IN v8.5.0 — GREATEST UPDATE ]</code>
</h2>

### 1. Braille Spinners Wrapped in `[ ]` with Osaka-Jade Colour

| Before | After |
|--------|-------|
| `⠋` plain spinner | `[⠋]` in osaka-jade bright `#50dca0` |
| `⠋ ⠏` dual spinners | `[⠋ ⠏]` in osaka-jade bright `#50dca0` |
| Gruvbox earthy tones | Lavender labels + osaka-jade accents |

- ScanBoard header spinner renders as `[⠋]` spinning in terminal
- AgentBar dual spinners render as `[⠋ ⠏]` during parallel agent execution
- Consistent `[bracket]` wrapping for all animated indicators

**Files:** `src/cli/display.rs`

---

### 2. Full Palette Migration — Gruvbox → Osaka-Jade & Lavender

All 20 `GB_*` (Gruvbox Evergreen) colour constants have been **removed entirely** and replaced with a clean semantic colour system.

| Component | Before (Gruvbox) | After (Osaka-Jade) |
|-----------|-----------------|-------------------|
| Palette base | Olive / brown earth tones | Deep navy `#1a1a2e` |
| Primary accent | `GB_GRN_B` `#b8bb26` | `OSAKA_JADE_B` `#50dca0` |
| Labels | `GB_GRY` `#928374` | `COL_DIM` `#788298` |
| Critical | `GB_RED_B` `#fb4934` | `COL_CRIT` `#ff3232` |
| High | `GB_RED` `#cc241d` | `COL_HIGH` `#ff6450` |
| Medium | `GB_YLW` `#d79921` | `COL_MED` `#ffb432` |
| Low | `GB_ORG` `#fe8019` | `COL_LOW` `#f0a030` |
| Info | `GB_BLU` `#458588` | `COL_INFO` `#aac3eb` |
| Dim / secondary | — | `COL_DIM` `#788298` |
| Title / labels | — | Lavender `#beb0eb` |
| Body text | — | Lavender-Blue `#aac3eb` |

```
[CRITICAL] → #ff3232  (bright red)
[  HIGH  ] → #ff6450  (warm orange-red)
[ MEDIUM ] → #ffb432  (golden amber)
[  LOW   ] → #f0a030  (warm orange)
[  INFO  ] → #aac3eb  (soft lavender-blue)
```

**Files:** `src/cli/display.rs`, `src/main.rs`, `src/hybrid.rs`

---

### 3. Real-Time `det:` / `err:` Progress During Fuzzing

The progress display no longer freezes at `det:0  err:0` during URL processing.

- `prog_det` and `prog_err` atomics are updated **live** inside `fuzz_url()` after every detection and every network error
- The ScanBoard display ticks up in real time as findings accumulate
- Zero stale display — users see progress as it happens

**Files:** `src/hybrid.rs` (lines in `fuzz_url()` function)

---

### 4. Evidence = Real Response Body Content

Finding evidence for XSS, LFI, CMDi, and SSTI now carries **actual response body text** (first 200 bytes) instead of the bare string `"HTTP 200"`.

```rust
// Before (useless for confirmation)
evidence: "HTTP 200"

// After (meaningful for pattern matching)
evidence: "<html><script>alert('XSS')</script>..."
```

This enables the `Confirm` module to perform accurate string-based validation.

**Files:** `src/hybrid.rs`

---

### 5. Confirm::reduce_false_positive() Overhaul

| Change | Detail |
|--------|--------|
| Body evidence check | Findings with `evidence.len() > 10` and not containing `"HTTP "` pass through automatically |
| Severity preservation | Critical and High severity findings are always preserved |
| XSS patterns added | `<svg`, `<img`, `<iframe` detection |
| CMDi patterns added | `root:`, `nobody:` patterns in response |
| LFI patterns added | `daemon:x:`, `bin:x:` patterns in response |

**Files:** `src/detection/confirm.rs`

---

### 6. SQLi Regex Expansion

10 new SQL error detection patterns added to `matcher.rs`:

| Pattern | Detects |
|---------|---------|
| `mysqli_fetch` | PHP MySQLi fetch errors |
| `Unclosed quotation` | MS SQL unclosed quotes |
| `Incorrect syntax` | MS SQL syntax errors |
| `SQLSTATE` | Generic SQL state errors |
| `pg_query` | PostgreSQL query errors |
| `ODBC Driver` | ODBC driver errors |
| `Microsoft OLE DB` | OLE DB provider errors |
| `java.sql.*` | Java SQL exceptions |
| `Warning.*mysql` | MySQL warnings |
| `syntax error` | Generic SQL syntax errors |

**Files:** `src/detection/matcher.rs`

---

### 7. WAF Gate Tamed — Fewer False Positives

`is_waf_block()` now requires **BOTH** `"waf"` **AND** `"blocked"` / `"denied"` to appear together in the response.

- No longer triggers on common words like `"protection"`, `"challenge"`, or `"blocked"` appearing alone
- Applies to both `analyzer.rs` and `hybrid.rs` WAF detection paths
- Net result: fewer false negatives, more accurate WAF identification

**Files:** `src/detection/analyzer.rs`, `src/hybrid.rs`

---

### 8. Hypersecurity Kernel Module (`libhypersecurity.so`)

A memory-safety kernel module compiled as a separate `cdylib` workspace member:

| Export | Signature | Purpose |
|--------|-----------|---------|
| `hs_check_leaks` | `() -> bool` | Scans `/proc/self/maps` for W+X memory regions |
| `hs_sanitise_cache` | `() -> bool` | Writes to `/proc/sys/vm/drop_caches` |
| `hs_memory_barrier` | `() -> bool` | Issues `atomic_thread_fence(SeqCst)` |
| `hs_version` | `() -> *const c_char` | Returns version string `"8.5.0"` |

- Loaded at runtime via `libloading` — **zero-link dependency**
- Silently no-ops for non-root users (cache sanitisation)
- ~1.9 MB compiled binary

**Files:** `hypersecurity/Cargo.toml`, `hypersecurity/src/lib.rs`

---

### 9. Build Configuration & Cleanup

| Change | Detail |
|--------|--------|
| Workspace members | `hypersecurity` + `oxide-proxy` defined in `Cargo.toml` |
| Build jobs | `.cargo/config.toml` sets `jobs = 2` for constrained environments |
| Profile settings | All release profiles lifted to workspace root (no more warnings) |
| Zero warnings | `cargo check` and `cargo build --release -j2` pass with zero warnings |

**Files:** `Cargo.toml`, `.cargo/config.toml`

---

### 10. Duration Enforcement

The `--duration` flag now works precisely — no more 5-second grace period:

- Global duration timer before target loop in multiattack mode
- Per-payload deadline checks inside `fuzz_url()`
- Stops within ~1 second of the configured time limit
- Crawl phase checks deadline before and after each URL fetch

---

### 11. Request Counting Fixed

- `total_reqs` now properly accumulates `scanner.req_count` per target in multiattack mode
- SCAN COMPLETE displays actual HTTP request count (not finding count)
- `HybridScanner.req_count` is an `AtomicUsize` tracking every request in real time

---

### 12. Findings Always Printed After Scan

- Removed the `args.verbose` gate that suppressed findings in non-verbose mode
- Removed the `findings.len() < 5` shortcut
- SCAN COMPLETE uses ANSI-aware `vis()` for correct right-padding
- All findings display unconditionally at scan end

---

### 13. CE Phase Deadline Checks

8 new `check_timeout!()` macro calls inserted across all CE scan phases:

```
RECON · TLS · CORS · COMMON · CREDS · PARAMS · FILTER · INSTA · SESSION
ML · Agent · Parallel · Body
```

Total phases with deadline enforcement: **11**

---

<h2 style="color:#50dca0;border-bottom:1px solid #50dca066;">
  <code>[ VULNERABILITY SCANNERS ]</code>
</h2>

<table>
<tr><th width="180">Module</th><th width="100">Flag</th><th>Description</th></tr>

<tr>
<td><code style="color:#ff3232;">SQL Injection</code></td>
<td><code>sqli</code></td>
<td>Error-based, blind boolean, time-based blind SQLi detection with 20+ regex patterns</td>
</tr>

<tr>
<td><code style="color:#ff6450;">Blind SQLi</code></td>
<td><code>blind-sqli</code></td>
<td>Timing-based blind SQLi with statistical response time analysis</td>
</tr>

<tr>
<td><code style="color:#ff6450;">XSS</code></td>
<td><code>xss</code></td>
<td>Reflected, stored, and DOM-based cross-site scripting detection</td>
</tr>

<tr>
<td><code style="color:#ffb432;">LFI</code></td>
<td><code>lfi</code></td>
<td>Local File Inclusion with path traversal chain mutation</td>
</tr>

<tr>
<td><code style="color:#ffb432;">Path Traversal</code></td>
<td><code>path-traversal</code></td>
<td>Directory traversal across Linux/Windows path variants</td>
</tr>

<tr>
<td><code style="color:#f0a030;">Command Injection</code></td>
<td><code>cmd-injection</code></td>
<td>OS command injection with blind and reflected detection</td>
</tr>

<tr>
<td><code style="color:#f0a030;">CORS</code></td>
<td><code>cors</code></td>
<td>Cross-Origin Resource Sharing misconfiguration assessment</td>
</tr>

<tr>
<td><code style="color:#aac3eb;">TLS Audit</code></td>
<td><code>tls</code></td>
<td>Full TLS/SSL security audit — protocols, ciphers, certificates</td>
</tr>

<tr>
<td><code style="color:#aac3eb;">Default Creds</code></td>
<td><code>creds</code></td>
<td>6000+ default credential combinations against common login endpoints</td>
</tr>

<tr>
<td><code style="color:#aac3eb;">Common Apps</code></td>
<td><code>common</code></td>
<td>Nikto-style common application checks — 2790+ tests</td>
</tr>

<tr>
<td><code style="color:#788298;">DB Fingerprint</code></td>
<td><code>db-fingerprint</code></td>
<td>Database engine fingerprinting via error messages and banner grabbing</td>
</tr>

<tr>
<td><code style="color:#788298;">Instagram OSINT</code></td>
<td><code>insta</code></td>
<td>Instagram profile intelligence — followers, private status, profile picture</td>
</tr>

<tr>
<td><code style="color:#788298;">Session Hijack</code></td>
<td><code>session</code></td>
<td>Cookie security flags, session fixation, token predictability</td>
</tr>

<tr>
<td><code style="color:#50dca0;">ML Trainer</code></td>
<td><code>train</code></td>
<td>Train Random Forest / SVM classifier from live scanner results</td>
</tr>

<tr>
<td><code style="color:#50dca0;">Zero-Day ML</code></td>
<td><code>zeroday</code></td>
<td>Anomaly detection via statistical modelling — <code>smartcore</code> + <code>linfa</code></td>
</tr>

<tr>
<td><code style="color:#50dca0;">Hypersecurity</code></td>
<td><code>hypersecurity</code></td>
<td>Kernel-level memory safety & cache sanitisation (shared library)</td>
</tr>
</table>

---

<h2 style="color:#50dca0;border-bottom:1px solid #50dca066;">
  <code>[ AI / ML ENGINE ]</code>
</h2>

| Component | Library | Purpose |
|-----------|---------|---------|
| **Zero-Day Detection** | `smartcore` (Random Forest, SVM) | Statistical anomaly detection on response patterns |
| **Pattern Learner** | Custom `ngram` analyser | Adaptive payload mutation from response analysis |
| **Exploit Analyzer** | Custom heuristic engine | AI-driven exploit chain analysis |
| **Response Analyzer** | Custom behavioural model | HTTP response behavioural fingerprinting |
| **Payload Mutator** | Custom genetic algorithm | ML-guided payload evolution |
| **linfa Clustering** | `linfa-clustering` | Unsupervised anomaly grouping via K-Means / DBSCAN |
| **Stats Engine** | `statrs` | Statistical distribution modelling for outlier detection |

---

<h2 style="color:#50dca0;border-bottom:1px solid #50dca066;">
  <code>[ ARCHITECTURE ]</code>
</h2>

```
oxide-v8.5.0/
├── src/
│   ├── main.rs              # Entry point, arg parsing, scan orchestration
│   ├── lib.rs               # Module tree and common exports
│   ├── hybrid.rs            # Core scan orchestration — fuzzing, multiattack, duration
│   ├── crawls.rs            # Web crawling and link extraction
│   ├── db.rs                # Encrypted SQLite database (XOR + magic header)
│   ├── filter.rs            # Response filtering and normalisation
│   ├── recon.rs             # Network recon (pnet raw TCP on Linux, HTTP passive on Windows)
│   │
│   ├── cli/
│   │   ├── args.rs          # Clap CLI argument definitions
│   │   ├── display.rs       # ScanBoard UI, colours, spinners, finding output
│   │   └── parser.rs        # Custom argument parser
│   │
│   ├── detection/
│   │   ├── analyzer.rs      # Finding struct, Severity enum, WAF detection
│   │   ├── confirm.rs       # False positive reduction, evidence validation
│   │   ├── matcher.rs       # SQLi/LFI/XSS/CMDi regex detection patterns
│   │   └── confirm.rs       # Timing analysis for blind vulnerabilities
│   │
│   ├── scanner/
│   │   ├── sqli_scanner.rs  # SQL injection scanner
│   │   ├── xss_scanner.rs   # Cross-site scripting scanner
│   │   ├── lfi_scanner.rs   # Local file inclusion scanner
│   │   ├── cmdi_scanner.rs  # Command injection scanner
│   │   ├── cors_scanner.rs  # CORS misconfiguration scanner
│   │   ├── tls_scanner.rs   # TLS/SSL audit scanner
│   │   ├── creds_scanner.rs # Default credential brute-forcer (6000+ combos)
│   │   ├── common_scanner.rs# Nikto-style common checks (2790+ tests)
│   │   └── fingerprint.rs   # DB and server fingerprinting
│   │
│   ├── http/
│   │   ├── client.rs        # Async HTTP client (reqwest-based)
│   │   ├── proxy.rs         # Oxide-proxy library loader
│   │   └── tls.rs           # TLS configuration and certificate handling
│   │
│   ├── ai/                  # ML models, pattern learners, exploit analysers
│   ├── advanced/            # Fuzzer, evasion engine, rate limiter, cluster
│   ├── zero_day/            # ML-driven zero-day anomaly detection + trainer
│   ├── payload/             # Payload generation, mutation, encoding
│   ├── report/              # JSON / HTML / CSV / XML report generators
│   ├── agent.rs             # Autonomous agent behaviour
│   ├── insta/               # Instagram OSINT scraping module
│   └── session_hijack/      # Session hijack testing module
│
├── hypersecurity/           # Kernel module — memory safety .so
│   ├── Cargo.toml
│   └── src/lib.rs
│
├── oxide-proxy/             # Proxy routing, rotation, authentication .so
├── dist/                    # Distribution package scripts
├── docs/
│   └── CHANGELOG.md         # Full change history
│
├── Cargo.toml               # Workspace root
├── .cargo/config.toml       # Build config (jobs = 2)
├── README.md                # Quick-start documentation
├── RELEASE.md               # Release notes (detailed)
├── RELEASE_NOTES.md         # Release notes (web-friendly)
├── GITHUB.md                # ← This file: comprehensive release documentation
├── LICENSE                  # Proprietary software license
└── ANNOUNCEMENT.txt         # Final release announcement
```

---

<h2 style="color:#50dca0;border-bottom:1px solid #50dca066;">
  <code>[ QUICK START ]</code>
</h2>

### Linux

```bash
# 1. Extract
unzip oxide-v8.5.0-linux.zip
cd oxide-v8.5.0-linux

# 2. Make executable
chmod +x oxide

# 3. Quick scan
./oxide --url https://target.com --modules all

# 4. Full scan with report
./oxide --url https://target.com \
  --modules all \
  --verbose \
  --output report.html \
  --format html \
  --duration 600

# 5. Targeted scan
./oxide --url https://target.com/page.php?id=1 \
  --modules sqli,xss,lfi \
  --payload-limit 20 \
  --exploitation-level 75

# 6. Multi-attack (up to 3 targets)
./oxide --url https://target1.com --url https://target2.com \
  --multiattack

# 7. Load hypersecurity module
cargo build -p hypersecurity --release
sudo cp target/release/libhypersecurity.so /usr/lib/
```

### Windows

```powershell
# 1. Extract
Expand-Archive oxide-v8.5.0-windows.zip
cd oxide-v8.5.0-windows

# 2. Basic scan (proxy DLL must be in same folder)
.\oxide.exe --url https://target.com --modules all --verbose

# 3. Save report
.\oxide.exe --url https://target.com --output report.json --format json
```

---

<h2 style="color:#50dca0;border-bottom:1px solid #50dca066;">
  <code>[ CLI REFERENCE ]</code>
</h2>

<table>
<tr><th>Flag</th><th>Default</th><th>Description</th></tr>

<tr><td><code>-u, --url</code></td><td><em>(required)</em></td><td>Target URL (up to 3 with <code>--multiattack</code>)</td></tr>
<tr><td><code>--modules</code></td><td><code>all</code></td><td>Comma-separated: <code>engine,static,agent,body,fingerprint,tls,common,cors,creds,insta,session,sqli,xss,lfi,db-fingerprint</code></td></tr>
<tr><td><code>-t, --threads</code></td><td><code>20</code></td><td>Concurrent worker threads (1-100)</td></tr>
<tr><td><code>--payload-limit</code> / <code>--payloads</code></td><td><code>50</code></td><td>Max payloads per test point</td></tr>
<tr><td><code>--exploitation-level</code> / <code>--exploitation</code></td><td><code>50</code></td><td>Aggression level (1-100)</td></tr>
<tr><td><code>--duration</code></td><td><code>0</code> (unlimited)</td><td>Max scan duration in seconds</td></tr>
<tr><td><code>-o, --output</code></td><td><em>stdout</em></td><td>Output file path</td></tr>
<tr><td><code>-f, --format</code></td><td><code>json</code></td><td>Report format: <code>json</code>, <code>html</code>, <code>csv</code>, <code>xml</code></td></tr>
<tr><td><code>--rate-limit</code></td><td><code>0</code> (unlimited)</td><td>Requests per second cap</td></tr>
<tr><td><code>--proxy</code></td><td><em>none</em></td><td>Route through proxy server (URL)</td></tr>
<tr><td><code>--user-agent</code></td><td><em>default</em></td><td>Custom User-Agent string</td></tr>
<tr><td><code>--cookie</code></td><td><em>none</em></td><td>Cookie string for authenticated scans</td></tr>
<tr><td><code>--header</code></td><td><em>none</em></td><td>Extra HTTP headers</td></tr>
<tr><td><code>--follow-redirects</code></td><td><code>false</code></td><td>Follow HTTP redirects</td></tr>
<tr><td><code>--max-redirects</code></td><td><code>10</code></td><td>Max redirect chain depth</td></tr>
<tr><td><code>--insecure</code></td><td><code>false</code></td><td>Skip SSL certificate verification</td></tr>
<tr><td><code>--crawl-depth</code></td><td><code>3</code></td><td>How deep the crawler goes</td></tr>
<tr><td><code>--max-pages</code></td><td><code>100</code></td><td>Max pages to crawl</td></tr>
<tr><td><code>--zeroday</code></td><td><code>false</code></td><td>Enable zero-day ML detection</td></tr>
<tr><td><code>--train</code></td><td><code>false</code></td><td>Train ML classifier from live results</td></tr>
<tr><td><code>--insta</code></td><td><code>false</code></td><td>Instagram OSINT module</td></tr>
<tr><td><code>--session</code></td><td><code>false</code></td><td>Session hijack testing</td></tr>
<tr><td><code>-v, --verbose</code></td><td><code>false</code></td><td>Full verbose output</td></tr>
<tr><td><code>--multiattack</code></td><td><code>false</code></td><td>Scan up to 3 targets concurrently</td></tr>
</table>

---

<h2 style="color:#50dca0;border-bottom:1px solid #50dca066;">
  <code>[ DISPLAY & STYLE SYSTEM ]</code>
</h2>

### Colour Palette

```rust
// Semantic severity colours — applied everywhere
COL_CRIT  = (255,  50,  50)  // #ff3232  — bright red for critical
COL_HIGH  = (255, 100,  80)  // #ff6450  — orange-red for high
COL_MED   = (255, 180,  50)  // #ffb432  — golden amber for medium
COL_LOW   = (240, 160,  48)  // #f0a030  — warm orange for low
COL_INFO  = (170, 195, 235)  // #aac3eb  — soft lavender-blue for info
COL_DIM   = (120, 130, 150)  // #788298  — muted grey for secondary text
OSAKA_JADE   = (60, 200, 140)   // #3cc88c  — primary osaka-jade
OSAKA_JADE_B = (80, 220, 160)   // #50dca0  — bright osaka-jade (accents)
LAVENDER     = (190, 175, 235)  // #beb0eb  — lavender titles
LAVENDER_BLUE= (170, 195, 235)  // #aac3eb  — lavender-blue body text
```

### Severity Badge Format

```
[CRITICAL] Arbitrary File Read via Path Traversal  // https://target.com/page
[  HIGH  ] Stored XSS in Comment Field             // https://target.com/post
[ MEDIUM ] Missing X-Frame-Options Header          // https://target.com/admin
[  LOW   ] Server Fingerprint: nginx 1.24.0        // https://target.com
[  INFO  ] TLS Certificate Expires in 30 Days      // https://target.com
```

- Severity labels centre-padded inside `[ ]` brackets (9 characters wide)
- Finding title in white bold
- URL after `//` in italic dim grey
- Evidence (if present) indented on next line

### Scan UI Components

| Component | Style | Description |
|-----------|-------|-------------|
| ScanBoard header | `[⠋]` osaka-jade bright | Scanning progress with spinner |
| AgentBar | `[⠋ ⠏]` osaka-jade bright | Dual agent execution spinner |
| Progress bar | `██████░░░░` osaka-jade | URL completion bar |
| Counters | `det:5 err:2` | Real-time finding/error counters |
| Section header | `─` osaka-jade bright border | Scan phase transitions |
| SCAN COMPLETE | `─` osaka-jade bright border | Summary with severity counts |

---

<h2 style="color:#50dca0;border-bottom:1px solid #50dca066;">
  <code>[ HYPERSECURITY KERNEL MODULE ]</code>
</h2>

The `hypersecurity` module is a standalone `cdylib` workspace member providing memory-safety primitives at the kernel level:

### Exports

```c
// Scan /proc/self/maps for W+X (writable+executable) memory regions
bool hs_check_leaks(void);

// Trigger kernel cache sanitisation via /proc/sys/vm/drop_caches
// (requires root — silently no-ops for unprivileged callers)
bool hs_sanitise_cache(void);

// Issue a SeqCst memory barrier via atomic_thread_fence
bool hs_memory_barrier(void);

// Return version string "8.5.0"
const char* hs_version(void);
```

### Loading

```rust
// At runtime — no compile-time dependency
unsafe {
    let lib = libloading::Library::new("libhypersecurity.so")?;
    let func: libloading::Symbol<unsafe extern "C" fn() -> bool> =
        lib.get(b"hs_check_leaks")?;
    let leaks = func();
}
```

### Build

```bash
cargo build -p hypersecurity --release
# Output: target/release/libhypersecurity.so (~1.9 MB)
```

---

<h2 style="color:#50dca0;border-bottom:1px solid #50dca066;">
  <code>[ SECURITY HARDENING ]</code>
</h2>

| Feature | Description |
|---------|-------------|
| **XOR Encryption** | SQLite database encrypted with version-tied XOR key |
| **Magic Header Verification** | Decrypted temp file validated against known header |
| **Temp File Cleanup** | Decrypted database deleted immediately after loading |
| **Proxy FFI Sandbox** | `oxide-proxy` compiled as separate unit with `panic=abort` |
| **Runtime Enforcement** | Binary refuses to start without proxy library |
| **Hypersecurity .so** | Runtime W+X memory region scanning |
| **Cache Sanitisation** | Kernel page cache drop via `/proc/sys/vm/drop_caches` |
| **Proprietary License** | Author name and brand legally protected |

---

<h2 style="color:#50dca0;border-bottom:1px solid #50dca066;">
  <code>[ CODE QUALITY ]</code>
</h2>

- Zero Rust compiler warnings (`cargo check` and `cargo build --release -j2`)
- Zero `#[allow(dead_code)]` attributes
- All orphaned duplicate code removed
- No placeholder stubs, no `todo!()` macros
- Every module is real, working, production code
- Full palette migration — zero legacy Gruvbox references remain
- Workspace-wide profile configuration — no warnings on non-root packages

---

<h2 style="color:#50dca0;border-bottom:1px solid #50dca066;">
  <code>[ DISTRIBUTION PACKAGES ]</code>
</h2>

| Platform | File | Size | Contents |
|----------|------|------|----------|
| **Linux** | `oxide-v8.5.0-linux.zip` | ~6.2 MB | `oxide`, `liboxide_proxy.so`, `libhypersecurity.so`, `INSTALL.txt`, `README.md`, `LICENSE`, `RELEASE.txt`, `banner` |
| **Windows** | `oxide-v8.5.0-windows.zip` | ~5.2 MB | `oxide.exe`, `oxide_proxy.dll`, `INSTALL.txt`, `README.md`, `LICENSE`, `RELEASE.txt`, `banner` |

---

<h2 style="color:#50dca0;border-bottom:1px solid #50dca066;">
  <code>[ KNOWN LIMITATIONS ]</code>
</h2>

<table>
<tr><th>#</th><th>Limitation</th><th>Platform</th><th>Workaround</th></tr>
<tr><td>1</td><td><code>pnet</code> raw TCP recon</td><td>Linux only</td><td>Windows uses passive HTTP recon as fallback</td></tr>
<tr><td>2</td><td>Proxy library required at runtime</td><td>All</td><td>Place <code>liboxide_proxy.so</code> / <code>oxide_proxy.dll</code> next to binary</td></tr>
<tr><td>3</td><td><code>ring</code> crate + MSVC linker</td><td>Windows MSVC</td><td>Use <code>x86_64-pc-windows-gnu</code> (MinGW) for cross-compilation</td></tr>
<tr><td>4</td><td>hypersecurity cache sanitisation</td><td>Linux</td><td>Requires root — silently no-ops for unprivileged callers</td></tr>
</table>

---

<h2 style="color:#50dca0;border-bottom:1px solid #50dca066;">
  <code>[ FILES CHANGED IN v8.5.0 ]</code>
</h2>

<table>
<tr><th>File</th><th>Change</th></tr>
<tr><td><code>src/cli/display.rs</code></td><td>Braille in <code>[ ]</code> osaka-jade, full palette migration, GB_* → COL_*, thinner borders</td></tr>
<tr><td><code>src/cli/args.rs</code></td><td>Added <code>--duration</code> flag, <code>--payloads</code>/<code>--exploitation</code> aliases, thread default 20</td></tr>
<tr><td><code>src/hybrid.rs</code></td><td>Real-time det/err progress, body evidence, WAF gate fix, duration enforcement, GB_* → COL_*</td></tr>
<tr><td><code>src/main.rs</code></td><td>GB_* → COL_* imports, findings always printed, request counting fixed</td></tr>
<tr><td><code>src/detection/confirm.rs</code></td><td>Auto-pass body evidence, expanded XSS/LFI/CMDi patterns</td></tr>
<tr><td><code>src/detection/matcher.rs</code></td><td>10 new SQLi regex patterns</td></tr>
<tr><td><code>src/detection/analyzer.rs</code></td><td>WAF gate requires BOTH "waf" + "blocked"/"denied"</td></tr>
<tr><td><code>src/scanner/sqli_scanner.rs</code></td><td>New signature: <code>exploitation_level</code> + <code>silent_mode</code> parameters</td></tr>
<tr><td><code>Cargo.toml</code></td><td>Workspace with <code>hypersecurity</code> + <code>oxide-proxy</code> members</td></tr>
<tr><td><code>.cargo/config.toml</code></td><td>Build <code>jobs = 2</code> for constrained environments</td></tr>
</table>

### Files Added

<table>
<tr><th>File</th><th>Description</th></tr>
<tr><td><code>hypersecurity/Cargo.toml</code></td><td>Hypersecurity .so module manifest (<code>cdylib</code>)</td></tr>
<tr><td><code>hypersecurity/src/lib.rs</code></td><td>Memory safety & cache sanitisation kernel module (C ABI)</td></tr>
<tr><td><code>.cargo/config.toml</code></td><td>Build configuration (<code>jobs = 2</code>)</td></tr>
<tr><td><code>GITHUB.md</code></td><td>Comprehensive GitHub release documentation</td></tr>
</table>

---

<h2 style="color:#50dca0;border-bottom:1px solid #50dca066;">
  <code>[ LICENSE ]</code>
</h2>

**Proprietary Software License** — Copyright © 2024-2026 khaninkali · HyperSecurityLabs · All Rights Reserved

| Action | Public | HyperSecurity Members |
|--------|--------|----------------------|
| View source | ✅ Yes | ✅ Yes |
| Fork for reference | ✅ Yes | ✅ Yes |
| Personal / educational use | ✅ Yes | ✅ Yes |
| Compile and run | ✅ Yes | ✅ Yes |
| Modify code | ❌ No | ✅ Yes |
| Remove author attribution | ❌ **Never** | ❌ **Never** |
| Rebrand as own work | ❌ **Legal action** | ❌ **Legal action** |
| Sell / monetize | ❌ Written permission only | ❌ Written permission only |

> **Removing the author name ("khaninkali"), HyperSecurityLabs brand, or any copyright notice is a direct violation of this license and will result in legal action.**

See [LICENSE](./LICENSE) file for full terms.

---

<h2 style="color:#50dca0;border-bottom:1px solid #50dca066;">
  <code>[ LEGAL DISCLAIMER ]</code>
</h2>

OXIDE is intended **exclusively** for authorized penetration testing, security research, and educational purposes. You are solely responsible for ensuring you have proper authorization before scanning any target.

Misuse of this tool against systems you do not own or have explicit permission to test is illegal and unethical.

---

<h2 style="color:#50dca0;border-bottom:1px solid #50dca066;">
  <code>[ KALI LINUX ]</code>
</h2>

<p align="center">
  <img src="https://img.shields.io/badge/Kali_Linux-OXIDE_Ready-557C94?style=for-the-badge&logo=kali-linux&logoColor=white&labelColor=1a1a2e" />
</p>

OXIDE is **purpose-built and battle-tested for Kali Linux**. Whether you're running a full Kali install or a minimal Debian environment:

| Feature | Kali Support |
|---------|-------------|
| Raw TCP recon (`pnet`) | ✅ Full support (Linux-native) |
| Hypersecurity `.so` module | ✅ Native ELF loading |
| Proxy library (`liboxide_proxy.so`) | ✅ Native shared library |
| Cross-compilation target | ✅ `x86_64-unknown-linux-gnu` |
| Recommended install path | `/usr/local/bin/oxide` |
| Recommended module path | `/usr/local/lib/libhypersecurity.so` |

```bash
# Quick install on Kali
sudo cp oxide /usr/local/bin/
sudo cp libhypersecurity.so /usr/local/lib/
sudo cp liboxide_proxy.so /usr/local/lib/
oxide --url https://target.com --modules all
```

> **Kali Linux** is the industry-standard penetration testing distribution maintained by Offensive Security. OXIDE integrates seamlessly into any Kali workflow — no extra dependencies, no fighting with package managers.

---

<h2 style="color:#50dca0;border-bottom:1px solid #50dca066;">
  <code>[ ⭐ STAR THIS PROJECT ]</code>
</h2>

<p align="center">
  <a href="https://github.com/hypersecuritylabs/oxide-communityedition-v8.5.0">
    <img src="https://img.shields.io/badge/%E2%AD%90%20Star%20us%20on%20GitHub-50dca0?style=for-the-badge&labelColor=1a1a2e" alt="Star us on GitHub" />
  </a>
</p>

<p align="center" style="color:#aac3eb;">
  This is the <strong style="color:#50dca0;">final Community Edition release</strong> — free, open to view, and built with countless hours of work.
  <br/><br/>
  <span style="color:#788298;">
    If OXIDE has helped you in a pentest, CTF, or research project,<br/>
    <strong style="color:#50dca0;">please star the repository</strong> to show your support.
  </span>
  <br/><br/>
  <span style="color:#50dca0;">Every star fuels future Pro Edition development. ★</span>
</p>

---

<h2 style="color:#50dca0;border-bottom:1px solid #50dca066;">
  <code>[ CONNECT ]</code>
</h2>

<p align="center">

| Platform | Link |
|----------|------|
| 🐙 **GitHub** | [github.com/hypersecuritylabs](https://github.com/hypersecuritylabs) |
| 🌐 **Website** | [hypersecuritylabs.netlify.app](https://hypersecuritylabs.netlify.app) |
| 💬 **Telegram** | [t.me/hypersecurity_offsec](https://t.me/hypersecurity_offsec) |
| 🐉 **Kali Linux** | [kali.org/tools](https://www.kali.org/tools/) |

</p>

---

> **Special thanks to [lyara](https://github.com/lyara) for development contributions.**

<br/>

<p align="center">
  <code style="color:#50dca0;background:#1a1a2e;padding:8px 20px;border-radius:6px;border:1px solid #50dca066;">
    Built with 🦀 Rust · Forged in the offensive security trenches
  </code>
  <br/><br/>
  <strong style="color:#beb0eb;">HyperSecurityLabs · OXIDE Framework v8.5.0</strong>
  <br/>
  <span style="color:#788298;"><em>"Scan everything. Trust nothing. Patch accordingly."</em></span>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/END_OF_LINE-50dca0?style=for-the-badge&labelColor=1a1a2e" />
  <img src="https://img.shields.io/badge/FINAL_RELEASE-ff6450?style=for-the-badge&labelColor=1a1a2e" />
</p>
