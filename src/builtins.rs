extern crate nix;

use std::env::home_dir;
use nix::unistd;
use std::io::{self, Write};
use std::process;

pub fn cd(args: &[&str]) -> nix::Result<()> {
    let path = match args.get(0) {
        Some(path) => path.into(),
        None => home_dir().unwrap()
    };

    unistd::chdir(&path)
}

pub fn echo(args: &[&str]) -> nix::Result<()> {
    println!("{}", args[1..].join(" "));
    io::stdout().flush().unwrap();
    Ok(())
}

pub fn exit(args: &[&str]) -> nix::Result<()> {
    let status: i32 = args.get(0)
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);

    process::exit(status);
}
