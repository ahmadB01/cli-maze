use crate::error::{GameError, GameResult};
use rand::seq::SliceRandom;
use std::fs::{read_dir, DirEntry, OpenOptions};
use std::io::{self, Read, Result};
use std::path::{Path, PathBuf};

pub fn read_file(path: &Path) -> io::Result<String> {
    let mut file = OpenOptions::new().read(true).open(path)?;
    let mut out = String::new();
    file.read_to_string(&mut out)?;
    Ok(out)
}

pub fn random_map(path: &Path) -> GameResult<PathBuf> {
    let mut rng = rand::thread_rng();

    let files = read_dir(path)
        .map_err(|_| GameError::MapsDirNotFound(path.to_path_buf()))?
        .collect::<Result<Vec<DirEntry>>>()
        .map_err(|e| GameError::IoError(Some(path.to_path_buf()), e))?;

    let file = files
        .choose(&mut rng)
        .ok_or(GameError::NoMapFound(path.to_path_buf()))?;

    Ok(file.path())
}
