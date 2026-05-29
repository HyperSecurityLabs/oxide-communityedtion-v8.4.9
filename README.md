**OXIDE** (Open eXtensible Intelligence & Detection Engine) is a high-performance, AI-augmented web vulnerability scanner written entirely in **Rust**. It combines raw systems-level performance with machine learning-driven detection to find vulnerabilities that traditional scanners miss.

<p align="center">
  <img src="https://img.shields.io/badge/version-8.5.0-50dca0?style=for-the-badge&labelColor=1a1a2e" />
  <img src="https://img.shields.io/badge/status-FINAL%20RELEASE-80dca0?style=for-the-badge&labelColor=1a1a2e" />
  <img src="https://img.shields.io/badge/license-Proprietary-beb0eb?style=for-the-badge&labelColor=1a1a2e" />
  <img src="https://img.shields.io/badge/plat-Linux%20%7C%20Win-aac3eb?style=for-the-badge&labelColor=1a1a2e" />
  <img src="https://img.shields.io/badge/Rust-2021-50dca0?style=for-the-badge&labelColor=1a1a2e" />
  <img src="https://img.shields.io/badge/Kali_Linux-557C94?style=for-the-badge&logo=kali-linux&logoColor=white&labelColor=1a1a2e" />
</p>

```text

  ▷ This is the last freely-available Community Edition.
  Future development moves exclusively to OXIDE Pro Edition.

```

<p align="center">
  <a href="https://github.com/hypersecuritylabs/oxide-communityedition-v8.5.0">
    <img src="https://img.shields.io/badge/%E2%AD%90%20Star%20on%20GitHub-50dca0?style=for-the-badge&labelColor=1a1a2e" />
  </a>
  <a href="https://www.kali.org/tools/">
    <img src="https://img.shields.io/badge/Proudly%20crafted%20for-Kali%20Linux-557C94?style=for-the-badge&labelColor=1a1a2e&logo=kali-linux" />
  </a>
</p>

> **OXIDE** is a next-generation, AI-augmented web vulnerability scanner in **Rust**.  
> From SQLi/XSS to zero-day anomaly detection via Random Forest & SVM — built for offensive security pros.  
> **Release:** 2026-05-29 · **Author:** [khaninkali](https://github.com/hypersecuritylabs) · HyperSecurityLabs

---

![](https://img.shields.io/badge/WHAT'S%20NEW-v8.5.0-50dca0?style=flat-square)

**Braille in `[ ]`** — ScanBoard `[⠋]` / AgentBar `[⠋ ⠏]` in osaka-jade bright `#50dca0` · `src/cli/display.rs`

**Full palette migration** — All 20 Gruvbox `GB_*` constants removed. Replaced with semantic aliases:
```
[CRITICAL] → #ff3232 | [  HIGH  ] → #ff6450 | [ MEDIUM ] → #ffb432
[  LOW   ] → #f0a030 | [  INFO  ] → #aac3eb | COL_DIM → #788298
OSAKA_JADE_B = #50dca0 | LAVENDER = #beb0eb | LAVENDER_BLUE = #aac3eb
```
`src/cli/display.rs` · `src/main.rs` · `src/hybrid.rs`

**Real-time `det:`/`err:` progress** — `prog_det`/`prog_err` atomics update live inside `fuzz_url()` after each finding and network error. No more frozen `det:0 err:0`. `src/hybrid.rs`

**Evidence = real response body** — XSS/LFI/CMDi/SSTI findings carry first 200 bytes of actual response body instead of bare `"HTTP 200"`. Enables accurate confirmation. `src/hybrid.rs`

**Confirm filter overhaul** — auto-pass evidence >10 chars, preserve Critical/High by default, new patterns: `<svg` `<img` `<iframe` (XSS), `root:` `nobody:` (CMDi), `daemon:x:` `bin:x:` (LFI). `src/detection/confirm.rs`

**10 new SQLi regex patterns** — `mysqli_fetch`, `Unclosed quotation`, `Incorrect syntax`, `SQLSTATE`, `pg_query`, `ODBC Driver`, `Microsoft OLE DB`, `java.sql.*`, `Warning.*mysql`, `syntax error`. `src/detection/matcher.rs`

**WAF gate tamed** — `is_waf_block()` requires BOTH `"waf"` AND `"blocked"/"denied"` together. No longer triggers on `"protection"`, `"challenge"`, or `"blocked"` alone. `src/detection/analyzer.rs` · `src/hybrid.rs`

**Hypersecurity kernel module** — standalone `cdylib` workspace member. C ABI: `hs_check_leaks` (`/proc/self/maps` W+X scan), `hs_sanitise_cache` (drop_caches), `hs_memory_barrier` (SeqCst fence), `hs_version`. Loaded via `libloading` — zero-link dependency. `hypersecurity/`

**Build config** — workspace with `hypersecurity` + `oxide-proxy`, `.cargo/config.toml` sets `jobs=2`, all profiles lifted to workspace root, zero compiler warnings

**Duration enforcement** — `--duration` stops within ~1s of limit, per-payload deadline checks inside `fuzz_url()`, crawl phase gates before/after each URL

**Request counting fixed** — `total_reqs` accumulates `scanner.req_count` per target in multiattack. SCAN COMPLETE shows actual HTTP request count via `AtomicUsize`, not finding count

**Findings always printed** — removed `args.verbose` gate and `findings.len() < 5` shortcut. SCAN COMPLETE uses ANSI-aware `vis()` for correct right-padding.

**13-phase deadline checks** — `check_timeout!()` across RECON · TLS · CORS · COMMON · CREDS · PARAMS · FILTER · INSTA · SESSION · ML · Agent · Parallel · Body

---

![](https://img.shields.io/badge/SCANNERS-ffb432?style=flat-square)

| Module | Flag | Description |
|--------|------|-------------|
| SQL Injection | `sqli` | Error-based, blind, time-based with 20+ regex |
| Blind SQLi | `blind-sqli` | Timing analysis |
| XSS | `xss` | Reflected, stored, DOM-based |
| LFI | `lfi` | Path traversal chains |
| Path Traversal | `path-traversal` | OS variants |
| Command Injection | `cmd-injection` | Blind + reflected |
| CORS | `cors` | Misconfiguration assessment |
| TLS Audit | `tls` | Protocols, ciphers, certs |
| Default Creds | `creds` | 6000+ combos |
| Common Apps | `common` | 2790+ Nikto-style checks |
| DB Fingerprint | `db-fingerprint` | Error + banner fingerprinting |
| Instagram OSINT | `insta` | Followers, private, profile pic |
| Session Hijack | `session` | Cookie flags, fixation, predictability |
| ML Trainer | `train` | RF/SVM from live results |
| Zero-Day ML | `zeroday` | `smartcore` + `linfa` anomaly detection |
| Hypersecurity | `hypersecurity` | Memory safety .so module |

---

![](https://img.shields.io/badge/AI%20ML-50dca0?style=flat-square)

| Component | Library | Purpose |
|-----------|---------|---------|
| Zero-Day Detection | `smartcore` RF/SVM | Statistical anomaly detection |
| Pattern Learner | Custom ngram | Adaptive payload mutation |
| Exploit Analyzer | Custom heuristic | Exploit chain analysis |
| Response Analyzer | Custom model | HTTP behavioural fingerprinting |
| Payload Mutator | Custom genetic alg | ML-guided evolution |
| Clustering | `linfa-clustering` | Unsupervised anomaly grouping |
| Stats Engine | `statrs` | Distribution outlier detection |

---

![](https://img.shields.io/badge/DISTRIBUTION-788298?style=flat-square)

| Platform | File | Size | Contents |
|----------|------|------|----------|
| 🐧 **Linux** | `oxide-v8.5.0-linux.zip` | ~6.2 MB | `oxide`, `liboxide_proxy.so`, `libhypersecurity.so`, `INSTALL.txt`, `README.md`, `LICENSE`, `RELEASE.txt`, `banner` |
| 🪟 **Windows** | `oxide-v8.5.0-windows.zip` | ~5.2 MB | `oxide.exe`, `oxide_proxy.dll`, `INSTALL.txt`, `README.md`, `LICENSE`, `RELEASE.txt`, `banner` |

---

![](https://img.shields.io/badge/ARCHITECTURE-50dca0?style=flat-square)

```
oxide/
├── src/main.rs          # Entry point
├── src/hybrid.rs        # Fuzzing, multiattack, duration
├── src/crawls.rs        # Link extraction
├── src/db.rs            # Encrypted SQLite
├── src/cli/             # args.rs, display.rs, parser.rs
├── src/detection/       # analyzer, confirm, matcher
├── src/scanner/         # sqli, xss, lfi, cmdi, cors, tls, creds, common
├── src/http/            # client, proxy loader, tls
├── src/ai/              # ML models, pattern learners
├── src/zero_day/        # RF/SVM anomaly detection
├── src/advanced/        # Fuzzer, evasion, rate limiter
├── src/payload/         # Generation, mutation, encoding
├── src/report/          # JSON/HTML/CSV/XML generators
├── src/insta/           # Instagram OSINT
├── src/session_hijack/  # Session testing
├── hypersecurity/       # C ABI .so — memory safety
└── oxide-proxy/         # Proxy routing .so
```

---

![](https://img.shields.io/badge/QUICK%20START-aac3eb?style=flat-square)

**Linux:**
```bash
unzip oxide-v8.5.0-linux.zip && cd oxide-v8.5.0-linux && chmod +x oxide
./oxide --url https://target.com --modules all
./oxide --url https://target.com --modules all --verbose --output report.html --format html --duration 600
./oxide --url https://target.com/page.php?id=1 --modules sqli,xss,lfi --payload-limit 20 --exploitation-level 75
```

**Windows:**
```powershell
.\oxide.exe --url https://target.com --modules all --verbose
.\oxide.exe --url https://target.com --output report.json --format json
```

---

![](https://img.shields.io/badge/CLI%20REF-788298?style=flat-square)

| Flag | Default | Description |
|------|---------|-------------|
| `-u, --url` | *required* | Target URL (up to 3 with `--multiattack`) |
| `--modules` | `all` | Comma-separated module list |
| `-t, --threads` | `20` | Concurrent workers (1-100) |
| `--payload-limit` / `--payloads` | `50` | Max payloads per test |
| `--exploitation-level` / `--exploitation` | `50` | Aggression (1-100) |
| `--duration` | `0` (unlim) | Max scan seconds |
| `-o, --output` | stdout | Output file path |
| `-f, --format` | `json` | json/html/csv/xml |
| `--rate-limit` | unlimited | Req/sec cap |
| `--proxy` | none | Proxy URL |
| `--user-agent` | default | Custom UA |
| `--cookie` | none | Auth cookie |
| `--header` | none | Extra headers |
| `--follow-redirects` | false | Follow redirects |
| `--max-redirects` | `10` | Max chain depth |
| `--insecure` | false | Skip SSL verify |
| `--crawl-depth` | `3` | Crawler depth |
| `--max-pages` | `100` | Max crawl pages |
| `--zeroday` | false | ML zero-day mode |
| `--train` | false | Train classifier |
| `--insta` | false | Instagram OSINT |
| `--session` | false | Session hijack |
| `-v, --verbose` | false | Full output |
| `--multiattack` | false | Up to 3 targets |

---

![](https://img.shields.io/badge/PALETTE-50dca0?style=flat-square)

All 20 Gruvbox `GB_*` constants removed — replaced with semantic aliases:

| Component | Before (Gruvbox) | After (Osaka-Jade) |
|-----------|-----------------|-------------------|
| Primary accent | `GB_GRN_B` `#b8bb26` | `OSAKA_JADE_B` `#50dca0` |
| Labels | `GB_GRY` `#928374` | `COL_DIM` `#788298` |
| Critical | `GB_RED_B` `#fb4934` | `COL_CRIT` `#ff3232` |
| High | `GB_RED` `#cc241d` | `COL_HIGH` `#ff6450` |
| Medium | `GB_YLW` `#d79921` | `COL_MED` `#ffb432` |
| Low | `GB_ORG` `#fe8019` | `COL_LOW` `#f0a030` |
| Info | `GB_BLU` `#458588` | `COL_INFO` `#aac3eb` |
| Title text | — | Lavender `#beb0eb` |

**Finding format (one line):**
```
[CRITICAL] Arbitrary File Read via Path Traversal  // https://target.com/page
[  HIGH  ] Stored XSS in Comment Field             // https://target.com/post
[ MEDIUM ] Missing X-Frame-Options Header          // https://target.com/admin
[  LOW   ] Server Fingerprint: nginx 1.24.0        // https://target.com
[  INFO  ] TLS Certificate Expires in 30 Days      // https://target.com
```

**UI Components:**

| Component | Style | Description |
|-----------|-------|-------------|
| ScanBoard header | `[⠋]` osaka-jade bright | Scanning spinner |
| AgentBar | `[⠋ ⠏]` osaka-jade bright | Dual agent spinner |
| Progress | `████░░░░` osaka-jade | URL completion bar |
| Counters | `det:5 err:2` | Real-time finding/error count |
| Borders | `─` osaka-jade bright | Phase transitions, SCAN COMPLETE |

---

![](https://img.shields.io/badge/HYPERSECURITY-557C94?style=flat-square)

Standalone `cdylib` workspace member (~1.9 MB) — memory safety at kernel level. Loaded at runtime via `libloading` — zero-link dependency. Silently no-ops cache ops for non-root users.

**C ABI exports:**

```c
bool hs_check_leaks(void);      // /proc/self/maps W+X region scan
bool hs_sanitise_cache(void);   // /proc/sys/vm/drop_caches (requires root)
bool hs_memory_barrier(void);   // atomic_thread_fence(SeqCst)
const char* hs_version(void);   // returns "8.5.0"
```

**Runtime loading:**
```rust
unsafe {
    let lib = libloading::Library::new("libhypersecurity.so")?;
    let func: libloading::Symbol<unsafe extern "C" fn() -> bool> =
        lib.get(b"hs_check_leaks")?;
    let leaks = func();
}
```

**Build:**
```bash
cargo build -p hypersecurity --release
# target/release/libhypersecurity.so
```

---

![](https://img.shields.io/badge/HARDENING-ff6450?style=flat-square)

| Feature | Description |
|---------|-------------|
| XOR-encrypted SQLite | Database encrypted with version-tied XOR key |
| Magic header verify | Decrypted temp validated before use |
| Temp file cleanup | Decrypted DB deleted immediately after load |
| Proxy FFI sandbox | `oxide-proxy` separate compilation unit, `panic=abort` |
| Runtime enforcement | Binary won't start without proxy library |
| W+X scanning | `hs_check_leaks` monitors `/proc/self/maps` |
| Cache sanitisation | `hs_sanitise_cache` drops kernel page cache |
| Legal protection | Proprietary license — name/brand protected |

---

![](https://img.shields.io/badge/CODE%20QUALITY-50dca0?style=flat-square)

- Zero Rust compiler warnings (`cargo check` + `cargo build --release -j2` pass clean)
- Zero `#[allow(dead_code)]` attributes
- All orphaned duplicate code removed
- No placeholder stubs, no `todo!()` macros
- Every module is real, working, production code
- Full palette migration — zero legacy Gruvbox references remain

---

![](https://img.shields.io/badge/KALI%20LINUX-557C94?style=flat-square)

OXIDE is battle-tested on Kali Linux:

| Feature | Support |
|---------|---------|
| Raw TCP recon (`pnet`) | ✅ Native |
| Hypersecurity `.so` | ✅ ELF loading |
| Proxy `.so` | ✅ Shared lib |
| Install | `/usr/local/bin/oxide` |

```
sudo cp oxide /usr/local/bin/ && sudo cp libhypersecurity.so /usr/local/lib/
```

---

![](https://img.shields.io/badge/KNOWN%20LIMITS-788298?style=flat-square)

| # | Limitation | Platform | Workaround |
|---|------------|----------|------------|
| 1 | `pnet` raw TCP recon | Linux only | Windows: passive HTTP recon fallback |
| 2 | Proxy .so/.dll required | All | Place next to binary |
| 3 | `ring` + MSVC linker | Windows MSVC | Use `x86_64-pc-windows-gnu` |
| 4 | Cache sanitisation | Linux | Root only — silently no-ops |

---

![](https://img.shields.io/badge/FILES%20CHANGED-beb0eb?style=flat-square)

| File | Change |
|------|--------|
| `src/cli/display.rs` | Braille `[]`, palette migration, GB_*→COL_*, thinner borders |
| `src/cli/args.rs` | `--duration`, `--payloads`/`--exploitation` aliases |
| `src/hybrid.rs` | det/err progress, body evidence, WAF fix, duration gates |
| `src/main.rs` | COL_* imports, findings always print, request count fix |
| `src/detection/confirm.rs` | Auto-pass evidence, expanded patterns |
| `src/detection/matcher.rs` | 10 new SQLi regex |
| `src/detection/analyzer.rs` | WAF gate: requires "waf" + "blocked"/"denied" |
| `src/scanner/sqli_scanner.rs` | `exploitation_level` + `silent_mode` params |
| `Cargo.toml` | Workspace members |
| `.cargo/config.toml` | `jobs=2` |

**Added:** `hypersecurity/Cargo.toml`, `hypersecurity/src/lib.rs`, `.cargo/config.toml`, `GITHUB.md`

---

![](https://img.shields.io/badge/LICENSE-ff6450?style=flat-square)

**Proprietary** — Copyright © 2024-2026 khaninkali · HyperSecurityLabs · All Rights Reserved

| Action | Public | Members |
|--------|--------|---------|
| View/fork/reference | ✅ | ✅ |
| Personal/edu use | ✅ | ✅ |
| Modify/distribute | ❌ | ✅ |
| Remove attribution | ❌ **Never** | ❌ **Never** |
| Sell/rebrand | ❌ Legal action | ❌ Legal action |

> Removing author name ("khaninkali"), HyperSecurityLabs brand, or copyright = violation + legal action.

---

![](https://img.shields.io/badge/STAR%20THIS%20PROJECT-50dca0?style=flat-square)

<p align="center">
  This is the <strong>final Community Edition</strong> — countless hours of work.  
  If OXIDE helped you in a pentest, CTF, or research,  
  <a href="https://github.com/hypersecuritylabs/oxide-communityedition-v8.5.0"><strong>please star the repository ★</strong></a>
  <br/><br/>
  <a href="https://github.com/hypersecuritylabs/oxide-communityedition-v8.5.0">
    <img src="https://img.shields.io/badge/%E2%AD%90%20Star%20on%20GitHub-50dca0?style=for-the-badge&labelColor=1a1a2e" />
  </a>
</p>

---

![](https://img.shields.io/badge/CONNECT-beb0eb?style=flat-square)

| Platform | Link |
|----------|------|
| 🐙 GitHub | [github.com/hypersecuritylabs](https://github.com/hypersecuritylabs) |
| 🌐 Website | [hypersecuritylabs.netlify.app](https://hypersecuritylabs.netlify.app) |
| 💬 Telegram | [t.me/hypersecurity_offsec](https://t.me/hypersecurity_offsec) |
| 🐉 Kali Linux | [kali.org/tools](https://www.kali.org/tools/) |

---

> **Special thanks to [lyara] for development contributions.**

<p align="center">
  <code>Built with 🦀 Rust · Forged in the offensive security trenches</code><br/>
  <strong>HyperSecurityLabs · OXIDE Framework v8.5.0</strong><br/>
  <em>"Scan everything. Trust nothing. Patch accordingly."</em>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/END_OF_LINE-50dca0?style=for-the-badge&labelColor=1a1a2e" />
  <img src="https://img.shields.io/badge/FINAL_RELEASE-ff6450?style=for-the-badge&labelColor=1a1a2e" />
</p>
