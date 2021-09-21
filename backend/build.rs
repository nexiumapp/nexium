use std::{fs::canonicalize, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=../frontend/node_modules");
    println!("cargo:rerun-if-changed=../frontend/package.json");
    println!("cargo:rerun-if-changed=../frontend/src");

    if std::env::var("PROFILE").unwrap() != "release".to_string() {
        return;
    }

    let status = Command::new("yarn")
        .arg("build")
        .current_dir(canonicalize("../frontend").unwrap())
        .status()
        .unwrap();

    println!("Yarn exited with: {:?}", status);
}
