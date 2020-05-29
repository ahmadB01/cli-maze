use std::convert::From;
use std::fmt;
use std::io;
use std::path::PathBuf;

pub type GameResult<T> = Result<T, GameError>;

pub enum GameError {
    TerminalError(crossterm::ErrorKind),
    IoError(Option<PathBuf>, std::io::Error),
    InvalidMapFile(PathBuf, char),
    MapFileNotFound(PathBuf),
    MapsDirNotFound(PathBuf),
    NoMapFound(PathBuf),
}

impl fmt::Debug for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            GameError::TerminalError(e) => format!("Unexpected terminal error: {}", e.to_string()),
            GameError::IoError(path, e) => {
                let mut output = format!("Unexpected IO Error: {}", e.to_string());
                if let Some(path) = path {
                    output.push_str(format!("\n\tPath: {:?}", path).as_str());
                }
                output
            }
            GameError::InvalidMapFile(path, c) => format!(
                "Invalid map file at: {:?}\n\tUnexpected token `{}`",
                path, c
            ),
            GameError::MapFileNotFound(path) => format!("Map file not found at: {:?}", path),
            GameError::MapsDirNotFound(path) => {
                format!("Directory containing maps file not found at: {:?}", path)
            }
            GameError::NoMapFound(path) => format!("No map found in directory: {:?}", path),
        };
        write!(f, "{}", c)
    }
}

impl From<crossterm::ErrorKind> for GameError {
    fn from(e: crossterm::ErrorKind) -> Self {
        GameError::TerminalError(e)
    }
}

impl From<io::Error> for GameError {
    fn from(e: io::Error) -> Self {
        GameError::IoError(None, e)
    }
}
