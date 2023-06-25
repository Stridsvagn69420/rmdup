use std::process::Command;
use std::path::Path;
use std::io;
use std::env;

fn main() {
    // ----- Target Triple -----
    println!("cargo:rustc-env=TARGET={}", env::var("TARGET").unwrap());

    // ----- Rust Version -----
    println!("cargo:rustc-env=CARGO_VERSION={}", version("cargo"));
    println!("cargo:rustc-env=RUSTC_VERSION={}", version("rustc"));

    // ----- Git Data -----
    if Path::new(".git").exists() {
        // Get the current branch name
        match git(&["symbolic-ref", "--short", "HEAD"]) {
            Ok(branch) => println!("cargo:rustc-env=GIT_BRANCH_TAG={}", branch),
            // Try to get tag if you're not on a branch
            Err(_) => {
                if let Ok(tag) = git(&["describe", "--tags", "--exact-match"]) {
                    println!("cargo:rustc-env=GIT_BRANCH_TAG={}", tag)
                }
            }
        }
        // Get Latest Commit Hash
        if let Ok(hash) = git(&["rev-parse", "--short", "HEAD"]) {
            println!("cargo:rustc-env=GIT_HASH={}", hash);
        };
    }
}

fn git(args: &[&str]) -> io::Result<String> {
    let output = Command::new("git")
    .args(args)
    .output()?;
    Ok(String::from_utf8(output.stdout).unwrap())
}

fn version(name: &str) -> String {
    let output = Command::new(name).arg("--version").output().unwrap();
    let raw = String::from_utf8(output.stdout).unwrap();
    raw.replace(&format!("{} ", name), "")
}