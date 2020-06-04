use crate::utils::{clear, map_name, maps_list};
use crate::{GameResult, MAPS_PATH};

use std::fs::canonicalize;
use std::io::{Stdin, Stdout, Write};
use std::path::{Path, PathBuf};

pub fn disp_list() -> GameResult<()> {
    let full = canonicalize(MAPS_PATH)?;
    println!("{}Path: {:?}:", clear(), full);

    for (i, file) in maps_list(full.as_path())?.iter().enumerate() {
        let name = map_name(&file.path())?;
        println!("{} - {}", i + 1, name);
    }

    Ok(())
}

pub fn ask_map(ask: &str, stdin: &Stdin, stdout: &mut Stdout) -> GameResult<PathBuf> {
    let maps = maps_list(Path::new(MAPS_PATH))?;
    loop {
        print!("{}", ask);
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

pub fn is_sure(ask: String, stdin: &Stdin, stdout: &mut Stdout) -> GameResult<bool> {
    println!("{}", ask);
    print!("Are you sure? (\"yes\"/\"no\"): ");
    stdout.flush()?;
    let mut out = String::new();
    stdin.read_line(&mut out)?;
    Ok(out.trim() == "yes")
}
