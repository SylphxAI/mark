fn main() {
    // Prefer explicit CI/platform injection, else git HEAD when building from a checkout.
    let sha = std::env::var("GIT_SHA")
        .or_else(|_| std::env::var("SOURCE_COMMIT"))
        .or_else(|_| std::env::var("SYLPHX_GIT_SHA"))
        .or_else(|_| std::env::var("COMMIT_SHA"))
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .or_else(|| {
            std::process::Command::new("git")
                .args(["rev-parse", "HEAD"])
                .output()
                .ok()
                .and_then(|o| {
                    if o.status.success() {
                        String::from_utf8(o.stdout).ok()
                    } else {
                        None
                    }
                })
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
        })
        .unwrap_or_else(|| "unknown".into());

    println!("cargo:rustc-env=MARK_GIT_SHA={sha}");
    println!("cargo:rerun-if-env-changed=GIT_SHA");
    println!("cargo:rerun-if-env-changed=SOURCE_COMMIT");
    println!("cargo:rerun-if-env-changed=SYLPHX_GIT_SHA");
    println!("cargo:rerun-if-env-changed=COMMIT_SHA");
    println!("cargo:rerun-if-changed=.git/HEAD");
}
