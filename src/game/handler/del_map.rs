use crate::game::handler::utils::{ask_map, disp_list, is_sure};
use crate::utils::map_name;
use crate::GameResult;

use std::fs::remove_file;
use std::io::{Stdin, Stdout};

pub fn run(stdin: &Stdin, stdout: &mut Stdout) -> GameResult<()> {
    disp_list()?;
    println!("--- Deleting a map ---\n");

    let map_path = ask_map(
        "Choose the map to remove by giving an index: ",
        stdin,
        stdout,
    )?;

    let name = map_name(&map_path)?;

    if is_sure(format!("Deleting map: {}", name), stdin, stdout)? {
        remove_file(map_path)?;
    }

    Ok(())
}
