use cli_maze::add_map;
use cli_maze::play;

use cli_maze::game::{Menu, Mode};
use cli_maze::utils::clear;
use std::io::{stdin, stdout, Write};
// the Menu! macro, it cannot be confused by the compiler
// with the Menu struct
use cli_maze::{GameResult, Menu};

fn main() -> GameResult<()> {
    // i will integrate more modes in the future
    // see cli-maze/game/handler :D
    let menu = Menu![
        #[field(display = "Play", perform = play::run)]
        Play,
        #[field(display = "Add a map", perform = add_map::run)]
        AddMap,
    ];

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
