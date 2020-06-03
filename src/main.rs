use cli_maze::{add_map, del_map, exit, play, ren_map};

use cli_maze::game::{Menu, Mode};
use cli_maze::utils::clear;
use std::io::{stdin, stdout, Stdin, Stdout, Write};
// the Menu! macro, it cannot be confused by the compiler
// with the Menu struct
use cli_maze::{GameResult, Menu};

// i will integrate more modes in the future
// see cli-maze/game/handler :D
fn menu() -> Menu<'static> {
    Menu![
        #[field(display = "Play", perform = play::run)]
        Play,
        #[field(display = "Add a map", perform = add_map::run)]
        AddMap,
        #[field(display = "Remove a map", perform = del_map::run)]
        DelMap,
        #[field(display = "Rename a map", perform = ren_map::run)]
        RenMap,
        #[field(display = "Exit game", perform = exit::run)]
        Exit,
    ]
}

fn menu_loop(menu: &Menu, stdin: &Stdin, stdout: &mut Stdout) -> GameResult<()> {
    loop {
        print!(">>> ");
        stdout.flush()?;
        let mut out = String::new();
        stdin.read_line(&mut out)?;
        match menu.perform(out, stdin, stdout) {
            Ok(_) => break Ok(()),
            Err(e) => {
                println!("{:?}", e);
                continue;
            }
        }
    }
}

fn main() -> GameResult<()> {
    let menu = menu();

    let stdin = stdin();
    let mut stdout = stdout();

    loop {
        print!("{}{}", clear(), menu);
        menu_loop(&menu, &stdin, &mut stdout)?;
    }
}
