use crate::error::{GameError, GameResult};
use crate::game::handler::utils::disp_list;
use crate::utils::maps_list;
use crate::MAPS_PATH;

use std::fs::remove_file;
use std::io::{stdin, stdout, Write};
use std::path::Path;

fn ask_idx() -> GameResult<usize> {
    let mut stdout = stdout();
    let stdin = stdin();

    loop {
        print!("Choose the map to remove by giving an index: ");
        stdout.flush()?;
        let mut out = String::new();
        stdin.read_line(&mut out)?;

        match out.trim().parse::<usize>() {
            Ok(idx) => break Ok(idx - 1),
            Err(_) => continue,
        };
    }
}

pub fn run() -> GameResult<()> {
    let maps = maps_list(Path::new(MAPS_PATH))?;

    disp_list()?;
    println!("--- Deleting a map ---\n");
    let idx = ask_idx()?;

    let map_path = maps.get(idx).ok_or(GameError::IncorrectInput)?.path();
    remove_file(map_path)?;

    Ok(())
}
