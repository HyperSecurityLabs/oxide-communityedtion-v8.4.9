use std::path::PathBuf;

fn main() {
    let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let profile = std::env::var("PROFILE").unwrap();
    let target = std::env::var("TARGET").unwrap_or_default();

    let is_windows = target.contains("windows");
    let lib_name = if is_windows { "oxide_proxy.dll" } else { "liboxide_proxy.so" };

    // Build the proxy library for the correct target
    let proxy_dir = manifest.join("oxide-proxy");
    let out_dir = manifest.join("target").join(&profile);

    let mut cmd = std::process::Command::new("cargo");
    cmd.args(["build", "--release"])
        .current_dir(&proxy_dir);

    if is_windows {
        cmd.arg("--target").arg(&target);
    }

    let status = cmd.status()
        .expect("failed to execute cargo build for oxide-proxy");
    assert!(status.success(), "oxide-proxy build failed");

    // Locate the built library
    let lib_src = if is_windows {
        let ws_target = manifest.join("target").join(&target).join("release");
        let p = ws_target.join(lib_name);
        if p.exists() {
            p
        } else {
            proxy_dir.join("target").join("release").join(lib_name)
        }
    } else {
        proxy_dir.join("target").join("release").join(lib_name)
    };

    let lib_dst = out_dir.join(lib_name);

    if lib_src.exists() {
        std::fs::copy(&lib_src, &lib_dst).expect(&format!("failed to copy {}", lib_name));
        let size = std::fs::metadata(&lib_dst).map(|m| m.len()).unwrap_or(0);
        println!("cargo:warning={} ({}) copied to target/{}/", lib_name, size, profile);
    } else {
        panic!("oxide-proxy build did not produce {} at {:?}", lib_name, lib_src);
    }
}
