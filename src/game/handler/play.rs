use crate::map::{Map, State};
use crate::utils::{clear, map_name, random_map};
use crate::{GameResult, MAPS_PATH};

use crossterm::event::{read, Event};
use std::io::{Stdin, Stdout};
use std::path::Path;

fn g_loop(mut game: Map) -> GameResult<()> {
    loop {
        println!("{}{}", clear(), game);

        if let Event::Key(e) = read()? {
            game.move_p(e.code);
        }

        if let State::InGame = game.get_state() {
            continue;
        } else {
            break Ok(());
        }
    }
}

pub fn run(stdin: &Stdin, stdout: &mut Stdout) -> GameResult<()> {
    let path = random_map(Path::new(MAPS_PATH))?;
    let game = Map::new(path.as_path(), stdin, stdout)?.with_name(map_name(&path)?);
    g_loop(game)?;
    read()?;
    Ok(())
}
