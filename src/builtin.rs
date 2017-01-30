extern crate nix;

use std::env::home_dir;
use nix::unistd;

pub fn cd<'a>(args: &'a Vec<&'a str>) -> nix::Result<()> {
    let path = match args.get(1) {
        Some(path) => path.into(),
        None => home_dir().unwrap()
    };

    unistd::chdir(&path)
}
