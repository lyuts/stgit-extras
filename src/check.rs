mod git;

use anyhow::{anyhow, Context};
use git::list_branches;
use std::process::{Command, Stdio};

fn stg_exists() -> anyhow::Result<()> {
    print!("Checking stg binary...");
    Command::new("stg")
        .arg("version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .map(|_| ())
        .context("stg binary not found")
}

fn dev_branch_exists() -> anyhow::Result<()> {
    print!("Detecting development branch...");
    let branches = list_branches()?;
    let maybe_branch: Option<String> = branches
        .iter()
        .filter(|b| {
            ["dev", "devel", "develop", "development"]
                .to_vec()
                .contains(&&b.as_str())
        })
        .collect::<Vec<&String>>()
        .first()
        .cloned()
        .cloned();

    match maybe_branch {
        Some(branch) => {
            print!(" {}", branch);
            Ok(())
        }
        None => Err(anyhow!("Unable to find development branch")),
    }
}

fn check<F>(fns: &[F]) -> anyhow::Result<()>
where
    F: Fn() -> anyhow::Result<()>,
{
    for f in fns {
        match f() {
            Ok(_) => println!(" Ok"),
            Err(e) => {
                println!(" Failed");
                eprintln!("{}", e);
                return Err(e);
            }
        }
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    check([stg_exists, dev_branch_exists].as_slice())?;
    Ok(())
}
