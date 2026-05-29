use std::path::PathBuf;

fn target_dir(manifest: &PathBuf) -> PathBuf {
    std::env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| manifest.join("target"))
}

fn main() {
    let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let profile = std::env::var("PROFILE").unwrap();
    let target = std::env::var("TARGET").unwrap_or_default();

    let is_windows = target.contains("windows");
    let is_macos = target.contains("apple");
    let lib_name = if is_windows {
        "oxide_proxy.dll"
    } else if is_macos {
        "liboxide_proxy.dylib"
    } else {
        "liboxide_proxy.so"
    };

    // Build the proxy library — always for host to keep it simple
    let proxy_dir = manifest.join("oxide-proxy");
    let out_dir = target_dir(&manifest).join(&profile);

    let mut cmd = std::process::Command::new("cargo");
    cmd.args(["build", "--release"])
        .current_dir(&proxy_dir);

    let status = cmd.status()
        .expect("failed to execute cargo build for oxide-proxy");
    assert!(status.success(), "oxide-proxy build failed");

    // Locate the built library — workspace members build into the root target dir
    let lib_src = target_dir(&manifest).join("release").join(lib_name);
    let lib_dst = out_dir.join(lib_name);

    if lib_src.exists() {
        std::fs::copy(&lib_src, &lib_dst).expect(&format!("failed to copy {}", lib_name));
        let size = std::fs::metadata(&lib_dst).map(|m| m.len()).unwrap_or(0);
        println!("cargo:warning={} ({}) copied to target/{}/", lib_name, size, profile);
    } else {
        panic!("oxide-proxy build did not produce {} at {:?}", lib_name, lib_src);
    }
}
