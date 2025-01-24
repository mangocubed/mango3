use std::process::Command;

fn main() {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .expect("could not get git commit hash");
    let git_commit_hash = String::from_utf8(output.stdout).expect("could not parse git commit hash");

    println!("cargo:rustc-env=GIT_COMMIT_SHORT_HASH={}", &git_commit_hash[0..7]);
    println!("cargo:rustc-env=GIT_COMMIT_HASH={}", git_commit_hash);
}
