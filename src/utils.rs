use crate::error::{GameError, GameResult};
use rand::seq::SliceRandom;
use std::fs::{read_dir, DirEntry, OpenOptions};
use std::io::{self, Read, Result};
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub fn read_file(path: &Path) -> io::Result<String> {
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
    format!("unknown {}", id)
}

pub fn map_name(path: PathBuf) -> String {
    String::from(path.file_stem().unwrap().to_str().unwrap())
}

pub fn clear() -> &'static str {
    // we cant center a text with ms-dos :'(
    "\x1b[2J\x1b[1;1H\
     CLI-MAZE\n\
     ----------------------------\n"
}
