extern crate nix;
extern crate ansi_term;
extern crate time;

use std::ffi::CString;
use std::io::{self, Write};
use nix::{unistd, sys};
use ansi_term::Colour::{Green, Red};
mod builtin;

fn main() {
    prompt(Ok(()));

    loop {
        let line = read_line().expect("can't read from stdin");
        prompt(run(&parse_args(&line)));
    }
}

fn prompt(status: nix::Result<()>) {
    let time = time::now();
    let cwd = unistd::getcwd().unwrap();
    let icon = match status {
        Ok(_) => Green.paint("v"),
        Err(_) => Red.paint("x"),
    };

    print!("{icon} {cwd} \n{time} $ ",
           icon=icon,
           time=time.strftime("%H:%M").unwrap(),
           cwd=cwd.to_str().unwrap());

    io::stdout().flush().unwrap()
}

fn read_line() -> io::Result<String> {
    let mut buffer = String::new();
    try!(io::stdin().read_line(&mut buffer));
    Ok(buffer)
}

fn parse_args<'a>(line: &'a String) -> Vec<&'a str> {
    line.split_whitespace().collect()
}

fn run<'a>(args: &[&str]) -> nix::Result<()> {

    match args.get(0) {
        Some(&"cd") => builtin::cd(args),
        Some(&"echo") => builtin::echo(args),
         _ => exec(args)
    }
}

fn exec<'a>(args: &[&str]) -> nix::Result<()> {
    let cargs: Vec<CString> = args.iter()
        .map(|s| CString::new(s.to_string()).unwrap())
        .collect();

    let result = try!(unistd::fork());
    match result {
        unistd::ForkResult::Parent { child } => {
            sys::wait::waitpid(child, None).map(|_| ())
        },
        _ => {
            try!(unistd::execvp(&cargs[0], &cargs));
            unreachable!()
        }
    }
}
