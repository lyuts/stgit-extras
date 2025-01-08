use anyhow::{anyhow, Context};
use std::process::{Command, ExitStatus};

pub fn list_branches() -> anyhow::Result<Vec<String>> {
    let output: Vec<u8> = Command::new("git")
        .arg("for-each-ref")
        .arg("--format='%(refname:short)'")
        .arg("refs/heads/")
        .output()
        .context("Unable to list branches.")?
        .stdout;

    let branches: Vec<String> = std::str::from_utf8(&output)?
        .trim()
        .split("\n")
        .map(|s| s.replace("'", ""))
        .collect();

    Ok(branches)
}

pub fn branch_exists(branch_name: &str) -> anyhow::Result<String> {
    let status_code: ExitStatus = Command::new("git")
        .arg("show-ref")
        .arg("--verify")
        .arg("--quiet")
        .arg(format!("refs/heads/{}", branch_name))
        .output()
        .context(format!("Branch {} does not exist.", branch_name))?
        .status;

    if status_code.success() {
        Ok(branch_name.to_string())
    } else {
        Err(anyhow!("Branch does not exist."))
    }
}
