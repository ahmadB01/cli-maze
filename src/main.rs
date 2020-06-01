use cli_maze::play;

use std::io::{stdin, stdout, Write};

use cli_maze::game::{Menu, Mode};
use cli_maze::utils::clear;
// the Menu! macro, it cannot be confused by the compiler
// with the Menu struct
use cli_maze::{GameResult, Menu};

fn main() -> GameResult<()> {
    // i will integrate more modes in the future
    // see cli-maze/game/handler :D
    let menu = Menu!(("Play", Play, play::run));

    print!("{}{}", clear(), menu);

    let stdin = stdin();
    let mut stdout = stdout();

    loop {
        print!(">>> ");
        stdout.flush()?;
        let mut out = String::new();
        stdin.read_line(&mut out)?;
        match menu.perform(out) {
            Ok(_) => break,
            Err(e) => {
                println!("{:?}", e);
                continue;
            }
        }
    }

    println!("Goodbye");
    Ok(())
}
