use cli_maze::error::GameResult;
use cli_maze::map::{Map, State};
use cli_maze::utils::random_map;
use crossterm::event::{read, Event};
use std::path::Path;

const MAPS_PATH: &str = "./maps/";

fn g_loop(mut game: Map) -> GameResult<State> {
    loop {
        clear();
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

fn clear() {
    print!("\x1b[2J\x1b[1;1H");
}

fn main() -> GameResult<()> {
    let path = random_map(Path::new(MAPS_PATH))?;
    let game = Map::new(path.as_path())?;
    let f_state = g_loop(game)?;

    if let State::Win = f_state {
        println!("youpi");
    } else {
        println!("t nul");
    }

    Ok(())
}
