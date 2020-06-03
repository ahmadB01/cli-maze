use crate::game::handler::utils::disp_list;
use crate::{GameResult, MAPS_PATH};
use std::fs::OpenOptions;
use std::io::{stdin, stdout, Write};

fn ask_name() -> GameResult<String> {
    loop {
        let mut out = String::new();
        print!("Enter the map name: ");
        stdout().flush()?;
        stdin().read_line(&mut out)?;
        let out = out.trim().to_owned();
        if !(out.is_empty() || out.contains(' ')) {
            break Ok(out);
        }
    }
}

fn get_raw() -> GameResult<String> {
    let mut out = String::new();
    loop {
        let mut line = String::new();
        stdin().read_line(&mut line)?;
        let line = line.trim().to_owned();
        if line.is_empty() {
            break Ok(out);
        } else {
            out = format!("{}{}\n", out, line);
        }
    }
}

fn make_file(name: String, raw: String) -> GameResult<()> {
    let path = format!("{}/{}.txt", MAPS_PATH, name);
    let mut file = OpenOptions::new().write(true).create(true).open(path)?;
    file.write_all(raw.as_bytes())?;
    Ok(())
}

pub fn run() -> GameResult<()> {
    disp_list()?;
    println!("--- Adding a map ---\n");
    let name = ask_name()?;
    let raw = get_raw()?;
    make_file(name, raw)?;
    Ok(())
}
