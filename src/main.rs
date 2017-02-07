#![feature(slice_patterns)]

extern crate nix;
extern crate ansi_term;
extern crate time;

use std::ffi::CString;
use std::io::{self, Write};
use nix::{unistd, sys};
use ansi_term::Colour::{Green, Red};
mod builtins;
mod commands;

use commands::Command;

fn main() {
    prompt(Ok(()));

    loop {
        let line = read_line().expect("can't read from stdin");
        let commands = commands::parse(&line);

        prompt(run(&commands));
    }
}

fn prompt(status: nix::Result<()>) {
    let time = time::now();
    let cwd = unistd::getcwd().unwrap();
    let icon = match status {
        Ok(_) => Green.paint("v"),
        Err(e) => Red.paint(format!("{}\nx", e.to_string())),
    };

    print!("{icon} {cwd} \n{time} $ ",
           icon=icon,
           time=time.strftime("%H:%M").unwrap(),
           cwd=cwd.to_str().unwrap());

    io::stdout().flush().unwrap()
}

fn read_line() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

fn run(commands: &[Command]) -> nix::Result<()> {
    for command in commands { exec(&command)? }
    sys::wait::wait().map(|_| ())
}

fn exec(command: &Command) -> nix::Result<()> {
    let result = unistd::fork()?;
    if result.is_parent() { return Ok(()); }

    if command.fd.0 != 0 {
        unistd::dup2(command.fd.0, 0)?;
        unistd::close(command.fd.0)?;
    }

    if command.fd.1 != 1 {
        unistd::dup2(command.fd.1, 1)?;
        unistd::close(command.fd.1)?;
    }

    match command.args.get(0) {
        Some(&"cd") => builtins::cd(&command.args),
        Some(&"echo") => builtins::echo(&command.args),
        _ => {
            let cargs: Vec<CString> = command.args.iter()
                .map(|s| CString::new(s.to_string()).unwrap())
                .collect();

            unistd::execvp(&cargs[0], &cargs)?;
            unreachable!()
        }
    }?;
    std::process::exit(0);
}
