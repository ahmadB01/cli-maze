use std::io::{Stdin, Stdout};

pub fn run(_: &Stdin, _: &mut Stdout) -> crate::GameResult<()> {
    println!("Goodbye!");
    std::process::exit(0)
}
