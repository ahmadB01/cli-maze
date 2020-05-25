pub mod bloc;
pub mod error;
pub mod map;
pub mod player;
pub mod utils;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point(usize, usize);
