use crate::build_errs;
use crossterm::ErrorKind;
use std::io::Error;
use std::path::PathBuf;

build_errs!(
    GameResult<Err=GameError>:
    Unexpected => String::from("An unexpected error occured"),
    IncorrectInput => String::from("This input is incorrect"),
    TerminalError(e: ErrorKind) => format!("Unexpected terminal error: {}", e),
    IoError(path: Option<PathBuf>, e: Error) => {
        let mut output = format!("Unexpected IO Error: {}", e);
        if let Some(path) = path {
            output.push_str(format!("\n\tPath: {:?}", path).as_str());
        }
        output
    },
    InvalidMapFile(path: PathBuf, c: char) => format!(
        "Invalid map file at: {:?}\n\tUnexpected token `{}`",
        path, c
    ),
    MapFileNotFound(path: PathBuf) => format!("Map file not found at: {:?}", path),
    MapsDirNotFound(path: PathBuf) => {
        format!("Directory containing maps files not found at: {:?}", path)
    },
    NoMapFound(path: PathBuf) => format!("No map found in directory: {:?}", path)
);
