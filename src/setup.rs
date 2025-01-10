mod git;

use std::process::Command;

use anyhow::{anyhow, Context};
use git::branch_exists;

fn ensure_dev_branch() -> anyhow::Result<String> {
    print!("Detecting development branch...");
    let dev_branch: Option<String> = branch_exists("dev")
        .or(branch_exists("devel"))
        .or(branch_exists("develop"))
        .or(branch_exists("development"))
        .ok();

    let default_dev_branch = "dev".to_string();
    println!(" {}", dev_branch.clone().unwrap_or_default());
    let main_branch = branch_exists("master").or(branch_exists("main"))?;
    if dev_branch.is_none() {
        print!(
            "Creating branch {} from origin/{} ...",
            default_dev_branch, main_branch
        );
        let child = Command::new("git")
            .arg("checkout")
            .arg("-b")
            .arg(&default_dev_branch)
            .arg(format!("origin/{}", main_branch))
            .output()
            .context("Failed to create development branch.")?;

        if child.status.success() {
            println!(" Done.");
        } else {
            return Err(anyhow!(
                "Failed to create branch {}:\n {}",
                default_dev_branch,
                std::str::from_utf8(&child.stderr)?
            ));
        }

        Command::new("git")
            .arg("checkout")
            .arg("-")
            .output()
            .context("Failed to switch back to the original branch.")?;
    }

    Ok(dev_branch.unwrap_or(default_dev_branch))
}

fn init_stgit(branch_name: String) -> anyhow::Result<()> {
    print!("Initializing stgit stack on {}...", branch_name);
    let child = Command::new("stg")
        .arg("init")
        .arg("-b")
        .arg(branch_name)
        .output()
        .context("Failed to initialize stgit stack.")?;
    if child.status.success() {
        println!(" Done.");
        Ok(())
    } else {
        Err(anyhow!("Failed to initialize stgit stack:\n{}", std::str::from_utf8(&child.stderr)?))
    }
}

fn main() -> anyhow::Result<()> {
    ensure_dev_branch().and_then(init_stgit)
}
