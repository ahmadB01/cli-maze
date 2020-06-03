use crate::utils::{clear, map_name, maps_list};
use crate::{GameResult, MAPS_PATH};

use std::fs::canonicalize;
use std::io::{Stdin, Stdout, Write};
use std::path::{Path, PathBuf};

pub fn disp_list() -> GameResult<()> {
    let full = canonicalize(MAPS_PATH)?;

    let maps = maps_list(full.as_path())?
        .iter()
        .enumerate()
        .map(|(i, file)| {
            let name = map_name(file.path());
            format!("{} - {}\n", i + 1, name)
        })
        .collect::<String>();

    println!("{}Path: {:?}:", clear(), full);
    println!("{}", maps);

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

pub fn is_sure(stdin: &Stdin, stdout: &mut Stdout) -> GameResult<bool> {
    print!("Are you sure? (\"yes\"/\"no\"): ");
    stdout.flush()?;
    let mut out = String::new();
    stdin.read_line(&mut out)?;
    Ok(out.trim() == "yes")
}

pub fn ask_name(ask: &str, stdin: &Stdin, stdout: &mut Stdout) -> GameResult<String> {
    loop {
        let mut out = String::new();
        print!("{}", ask);
        stdout.flush()?;
        stdin.read_line(&mut out)?;
        let out = out.trim().to_owned();
        if !(out.is_empty() || out.contains(' ')) {
            break Ok(out);
        }
    }
}
