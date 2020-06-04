use crate::error::{GameError, GameResult};

use rand::seq::SliceRandom;
use std::fs::{read_dir, DirEntry, OpenOptions};
use std::io::{Read, Result, Stdin, Stdout, Write};
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub fn read_file(path: &Path) -> Result<String> {
    let mut file = OpenOptions::new().read(true).open(path)?;
    let mut out = String::new();
    file.read_to_string(&mut out)?;
    Ok(out)
}

pub fn maps_list(path: &Path) -> GameResult<Vec<DirEntry>> {
    read_dir(path)
        .map_err(|_| GameError::MapsDirNotFound(path.to_path_buf()))?
        .collect::<Result<Vec<DirEntry>>>()
        .map_err(|e| GameError::IoError(Some(path.to_path_buf()), e))
}

pub fn random_map(path: &Path) -> GameResult<PathBuf> {
    let mut rng = rand::thread_rng();
    let files = maps_list(path)?;

    let file = files
        .choose(&mut rng)
        .ok_or_else(|| GameError::NoMapFound(path.to_path_buf()))?;

    Ok(file.path())
}

pub fn unknown_name() -> String {
    let id = Uuid::new_v4();
    format!("unknown_{}", id)
}

pub fn map_name(path: &Path) -> GameResult<String> {
    Ok(String::from(
        path.file_stem()
            .ok_or_else(|| GameError::NoMapFound(PathBuf::from(path)))?
            .to_str()
            .ok_or_else(|| GameError::Unexpected)?,
    ))
}

pub fn clear() -> &'static str {
    "\x1b[2J\x1b[1;1H\
     ####################\n\
     ##### CLI-MAZE #####\n\
     ####################\n\n"
}

pub fn ask_name(ask: &str, stdin: &Stdin, stdout: &mut Stdout) -> GameResult<String> {
    let mut out = String::new();
    print!("{}", ask);
    stdout.flush()?;
    stdin.read_line(&mut out)?;
    let out = out.trim().to_owned().replace(' ', "_");
    Ok(if out.is_empty() { unknown_name() } else { out })
}
