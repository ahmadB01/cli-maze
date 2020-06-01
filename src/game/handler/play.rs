use crate::map::{Map, State};
use crate::utils::{clear, map_name, random_map};
use crate::GameResult;
use crossterm::event::{read, Event};
use std::path::Path;

const MAPS_PATH: &str = "./maps/";

fn g_loop(mut game: Map) -> GameResult<State> {
    loop {
        print!("{}", clear());
        println!("{}", game);
        if let Event::Key(e) = read()? {
            game.move_p(e.code);
        }
        match game.get_state() {
            State::InGame => continue,
            _ => break Ok(game.f_state()),
        }
    }
}

pub fn run() -> GameResult<()> {
    let path = random_map(Path::new(MAPS_PATH))?;
    let game = Map::new(path.as_path())?.with_name(map_name(path));
    let f_state = g_loop(game)?;

    if let State::Win = f_state {
        println!("youpi");
    } else {
        println!("t nul");
    }
    Ok(())
}
