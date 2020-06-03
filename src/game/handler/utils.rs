use crate::utils::{clear, map_name, maps_list};
use crate::{GameResult, MAPS_PATH};

pub fn disp_list() -> GameResult<()> {
    let full = std::fs::canonicalize(MAPS_PATH)?;

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
