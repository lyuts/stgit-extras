mod git;

use std::process::{Command, Stdio};

use anyhow::{anyhow, Context};
use git::{branch_exists, list_branches};

fn ensure_dev_branch() -> anyhow::Result<String> {
    print!("Detecting development branch...");
    let mut branches: Vec<String> = list_branches()?
        .iter()
        .filter(|b| {
            ["dev", "devel", "develop", "development"]
                .to_vec()
                .contains(&&b.as_str())
        })
        .cloned()
        .collect::<Vec<String>>();

    branches.sort();

    let default_dev_branch = "dev".to_string();
    let dev_branch = branches.first().unwrap_or(&default_dev_branch);
    println!(" {}", dev_branch);
    let main_branch = branch_exists("master").or(branch_exists("main"))?;
    if branches.is_empty() {
        print!(
            "Creating branch {} from origin/{} ...",
            dev_branch, main_branch
        );
        let child = Command::new("git")
            .arg("checkout")
            .arg("-b")
            .arg(dev_branch)
            .arg(format!("origin/{}", main_branch))
            .output()
            .context("Failed to create development branch.")?;

        if child.status.success() {
            println!(" Done.");
        } else {
            return Err(anyhow!(
                "Failed to create branch {}:\n {}",
                dev_branch,
                std::str::from_utf8(&child.stderr)?
            ));
        }

        Command::new("git")
            .arg("checkout")
            .arg("-")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .context("Failed to switch back to the original branch.")?;
    }

    Ok(dev_branch.to_string())
}

fn init_stgit(branch_name: String) -> anyhow::Result<()> {
    print!("Initializing stgit stack on {}...", branch_name);
    let child = Command::new("stg")
        .arg("init")
        .arg("-b")
        .arg(branch_name)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .context("Failed to initialize stgit stack.")?;
    if child.status.success() {
        print!(" Done.");
        Ok(())
    } else {
        Err(anyhow!("Failed to initialize stgit stack:\n{}", std::str::from_utf8(&child.stderr)?))
    }
}

fn main() -> anyhow::Result<()> {
    ensure_dev_branch().and_then(init_stgit)
}
