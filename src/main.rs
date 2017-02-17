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
    let mut prev: Option<&Command> = None;

    let pids = commands.iter().map(|command| {
        if let Some(cmd) = prev {
            if cmd.fd.1 != 1 { unistd::close(cmd.fd.1).unwrap(); }
            if cmd.fd.0 != 0 { unistd::close(cmd.fd.0).unwrap(); }
        }

        let pid = exec(&command);
        prev = Some(&command);
        pid
    });

    for pid in pids {
        sys::wait::waitpid(pid.unwrap(), None).map(|_| ())?
    }

    Ok(())
}

fn exec(command: &Command) -> Result<i32, nix::Error> {
    let result = unistd::fork()?;
    if let nix::unistd::ForkResult::Parent { child } = result {
        return Ok(child);
    }

    if command.fd.0 != 0 {
        unistd::dup2(command.fd.0, 0).unwrap();
        unistd::close(command.fd.0).unwrap();
    }

    if command.fd.1 != 1 {
        unistd::dup2(command.fd.1, 1).unwrap();
        unistd::close(command.fd.1).unwrap();
    }

    let mut cargs: Vec<CString> = command.args.iter()
        .map(|s| CString::new(s.to_string()).unwrap())
        .collect();

    cargs.insert(0, CString::new(command.path.to_string()).unwrap());

    unistd::execvp(&cargs[0], &cargs)?;
    unreachable!()
}
