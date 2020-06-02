pub mod bloc;
pub mod error;
pub mod game;
pub mod map;
pub mod player;
pub mod utils;

pub use error::GameResult;
pub use game::handler::*;

pub const MAPS_PATH: &str = "./maps/";

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point(usize, usize);
