use cli_maze::error::{GameError, GameResult};
use cli_maze::map::Map;
use cli_maze::player::{Direction, Player};
use cli_maze::utils::random_map;
use crossterm::event::{read, Event, KeyCode};
use crossterm::{terminal, ExecutableCommand};
use std::io::{stdout, Write};
use std::path::Path;

const MAPS_PATH: &str = "./maps/";

fn get_nick() -> GameResult<String> {
    use std::io::{stdin, stdout, Write};

    print!(">>> Enter your nick: ");
    stdout().flush().map_err(|e| GameError::IoError(None, e))?;

    let mut out = String::new();
    stdin()
        .read_line(&mut out)
        .map_err(|e| GameError::IoError(None, e))?;
    let out = out.trim().to_owned();

    if out.is_empty() || out.contains(" ") {
        println!("Come on that's not funny.");
        get_nick()
    } else {
        println!("Ok {}.", out);
        Ok(out)
    }
}

fn g_loop(mut game: Map) -> GameResult<bool> {
    loop {
        clear();
        println!("{}", game);
        if let Event::Key(e) = read()? {
            game.move_p(e.code);
        }
    }
    Ok(false)
}

fn clear() {
    print!("\x1b[2J\x1b[1;1H");
}

fn main() -> GameResult<()> {
    clear();
    let player = Player::from(get_nick()?);
    let path = random_map(Path::new(MAPS_PATH))?;
    let game = Map::new(player, path.as_path())?;
    let won = g_loop(game)?;

    if won {
        println!("youpi");
    } else {
        println!("t nul");
    }

    Ok(())
}
