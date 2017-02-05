use std::os::unix::io::RawFd;
use nix::unistd;
use std::slice::Windows;



#[derive(Debug)]
pub struct Command<'a> {
    pub args: Vec<&'a str>,
    pub fd: (RawFd, RawFd),
}

fn parse_args(line: &str) -> Vec<&str> {
    line.split_whitespace().collect()
}

pub fn parse(line: &str) -> Vec<Command> {
    let mut commands: Vec<Command> = vec![];

    for (i, cmd) in line.split('|').enumerate() {
        let mut to = Command {
            args: parse_args(cmd),
            fd: (0, 1)
        };

        if i > 0 {
            if let Some(ref mut from) = commands.get_mut(i - 1) {
                let pipe = unistd::pipe().unwrap();
                from.fd.1 = pipe.1;
                to.fd.0 = pipe.0;
            }
        }


        commands.push(to);
    }

    commands
}
