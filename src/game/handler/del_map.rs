use crate::error::GameResult;
use crate::game::handler::utils::disp_list;
use crate::utils::maps_list;
use crate::MAPS_PATH;

use std::fs::{remove_file, DirEntry};
use std::io::{stdin, stdout, Stdin, Stdout, Write};
use std::path::{Path, PathBuf};

fn ask(maps: &[DirEntry], stdin: &Stdin, stdout: &mut Stdout) -> GameResult<PathBuf> {
    loop {
        print!("Choose the map to remove by giving an index: ");
        stdout.flush()?;
        let mut out = String::new();
        stdin.read_line(&mut out)?;

        let idx = match out.trim().parse::<usize>() {
            Ok(idx) => idx - 1,
            Err(_) => continue,
        };

        match maps.get(idx) {
            Some(entry) => break Ok(entry.path()),
            None => continue,
        }
    }
}

fn is_sure(stdin: &Stdin, stdout: &mut Stdout) -> GameResult<bool> {
    print!("Are you sure? (\"yes\"/\"no\"): ");
    stdout.flush()?;
    let mut out = String::new();
    stdin.read_line(&mut out)?;
    Ok(out.trim() == "yes")
}

pub fn run() -> GameResult<()> {
    let stdin = stdin();
    let mut stdout = stdout();

    let maps = maps_list(Path::new(MAPS_PATH))?;

    disp_list()?;
    println!("--- Deleting a map ---\n");
    let map_path = ask(&maps, &stdin, &mut stdout)?;

    if is_sure(&stdin, &mut stdout)? {
        remove_file(map_path)?;
    }

    Ok(())
}
