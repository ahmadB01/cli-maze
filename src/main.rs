use std::io::{stdin, stdout, Write};

use cli_maze::error::GameResult;
use cli_maze::game::{Menu, Mode};
use cli_maze::utils::clear;
// the cli_maze::game::mode::Menu macro
// so it cannot be confused with the cli_maze::game::main_menu::Menu struct
use cli_maze::Menu;

fn play() {
    println!("OK LETS PLAY XD");
}

fn add_map() {
    println!("ok you want to add a new map");
}

fn del_map() {
    println!("ok you want to delete a map");
}

fn main() -> GameResult<()> {
    let menu = Menu!(
        ("Play", Play, play),
        ("Add a map", AddMap, add_map),
        ("Remove a map", DelMap, del_map)
    );

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
            Err(_) => continue,
        }
    }

    println!("yallah bye");

    Ok(())
}
