use crate::game::handler::utils::{ask_name, disp_list};
use crate::{GameResult, MAPS_PATH};
use std::fs::OpenOptions;
use std::io::{Stdin, Stdout, Write};

fn get_raw(stdin: &Stdin) -> GameResult<String> {
    let mut out = String::new();
    loop {
        let mut line = String::new();
        stdin.read_line(&mut line)?;
        let line = line.trim().to_owned();
        if line.is_empty() {
            break Ok(out);
        } else {
            out = format!("{}{}\n", out, line);
        }
    }
}

fn create(name: String, raw: String) -> GameResult<()> {
    let path = format!("{}/{}.txt", MAPS_PATH, name);
    let mut file = OpenOptions::new().write(true).create(true).open(path)?;
    file.write_all(raw.as_bytes())?;
    Ok(())
}

pub fn run(stdin: &Stdin, stdout: &mut Stdout) -> GameResult<()> {
    disp_list()?;
    println!("--- Adding a map ---\n");
    let name = ask_name("Enter the map name", stdin, stdout)?;
    let raw = get_raw(stdin)?;
    create(name, raw)?;
    Ok(())
}
