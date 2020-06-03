use crate::game::handler::utils::{ask_map, ask_name, disp_list, is_sure};
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
    let map_path = ask_map(stdin, stdout)?;
    let to = rename_to(stdin, stdout)?;

    if is_sure(stdin, stdout)? {
        rename(map_path, to)?;
    }

    Ok(())
}
