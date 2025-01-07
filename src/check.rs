use anyhow::Context;
use std::process::{Command, Stdio};

fn stg_exists() -> anyhow::Result<()> {
    print!("Checking stg binary...");
    Command::new("stg")
        .arg("version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .map(|_| ())
        .context("stg binary not fond")
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
    check([stg_exists].as_slice())?;
    Ok(())
}
