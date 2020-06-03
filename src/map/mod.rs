mod bloc;
mod player;
mod state;
mod utils;

use player::Player;
pub use state::Map;

pub type Content = Vec<Vec<bloc::Bloc>>;

#[derive(Debug)]
pub enum State {
    InGame,
    Win,
    Loose,
}
