use crate::game::handler::utils::{ask_map, disp_list, is_sure};
use crate::utils::{ask_name, map_name};
use crate::{GameResult, MAPS_PATH};

use std::fs::rename;
use std::io::{Stdin, Stdout};
use std::path::PathBuf;

fn rename_to(stdin: &Stdin, stdout: &mut Stdout) -> GameResult<PathBuf> {
    let new = ask_name("Enter the new map name: ", stdin, stdout)?;
    let new = format!("{}/{}.txt", MAPS_PATH, new);
    Ok(PathBuf::from(new))
}

pub fn run(stdin: &Stdin, stdout: &mut Stdout) -> GameResult<()> {
    disp_list()?;
    let map_path = ask_map(
        "Choose the map to rename by giving an index: ",
        stdin,
        stdout,
    )?;
    let from_name = map_name(&map_path)?;

    let to = rename_to(stdin, stdout)?;
    let to_name = map_name(&to)?;

    if is_sure(
        format!("Renaming map {} to {}", from_name, to_name),
        stdin,
        stdout,
    )? {
        rename(map_path, to)?;
    }

    Ok(())
}
