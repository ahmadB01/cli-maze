use crate::utils::{clear, map_name, maps_list};
use crate::{GameResult, MAPS_PATH};
use std::fs;
use std::io::{self, Write};

fn disp_list() -> GameResult<()> {
    let full = fs::canonicalize(MAPS_PATH)?;

    let maps = maps_list(full.as_path())?
        .iter()
        .map(|file| {
            let name = map_name(file.path());
            format!("- {}\n", name)
        })
        .collect::<String>();

    println!("{}Path: {:?}:", clear(), full);
    println!("{}", maps);
    Ok(())
}

fn ask_name() -> GameResult<String> {
    loop {
        let mut out = String::new();
        print!("Enter the map name: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut out)?;
        let out = out.trim().to_owned();
        if !(out.is_empty() || out.contains(' ')) {
            break Ok(out);
        }
    }
}

pub fn run() -> GameResult<()> {
    disp_list()?;
    let name = ask_name()?;
    Ok(())
}
