use crate::game::handler::utils::{ask_map, disp_list, is_sure};
use crate::GameResult;

use std::fs::remove_file;
use std::io::{Stdin, Stdout};

pub fn run(stdin: &Stdin, stdout: &mut Stdout) -> GameResult<()> {
    disp_list()?;
    println!("--- Deleting a map ---\n");
    let map_path = ask_map(stdin, stdout)?;

    if is_sure(stdin, stdout)? {
        remove_file(map_path)?;
    }

    Ok(())
}
