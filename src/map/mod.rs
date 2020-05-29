mod state;
mod utils;

pub use state::Map;

pub type Content = Vec<Vec<crate::bloc::Bloc>>;

#[derive(Debug)]
pub enum State {
    InGame,
    Win,
    Loose,
}
