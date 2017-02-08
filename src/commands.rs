use std::os::unix::io::RawFd;
use nix::unistd;

#[derive(Debug)]
pub struct Command<'a> {
    pub path: &'a str,
    pub args: Vec<&'a str>,
    pub fd: (RawFd, RawFd)
}

impl<'a> Command<'a> {
    pub fn parse(cmd: &'a str) -> Option<Command> {
        let mut tokens = cmd.split_whitespace();
        tokens.next().map(|path| {
            Command {
                path: path,
                args: tokens.collect(),
                fd: (0, 1)
            }
        })

    }
}


pub fn parse(line: &str) -> Vec<Command> {
    let mut commands: Vec<Command> = vec![];

    for (i, cmd) in line.split('|').enumerate() {
        if let Some(mut to) = Command::parse(cmd) {
            if i > 0 {
                if let Some(ref mut from) = commands.get_mut(i - 1) {
                    let pipe = unistd::pipe().unwrap();
                    from.fd.1 = pipe.1;
                    to.fd.0 = pipe.0;
                }
            }


            commands.push(to);
        }
    }

    commands
}
