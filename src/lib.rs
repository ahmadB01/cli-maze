pub mod bloc;
pub mod error;
pub mod game;
pub mod map;
pub mod player;
pub mod utils;

pub use error::GameResult;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point(usize, usize);
