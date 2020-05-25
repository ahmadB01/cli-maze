use crate::Point;
use std::fmt;

#[derive(Debug)]
pub struct Bloc {
    pos: Point,
    kind: BlocKind,
    state: bool,
}

impl Bloc {
    pub fn new(pos: Point, kind: BlocKind) -> Self {
        Self {
            pos,
            kind,
            state: false,
        }
    }

    pub fn set_state(&mut self, update: bool) {
        self.state = update;
    }

    pub fn get_type(&self) -> &BlocKind {
        &self.kind
    }

    pub fn get_pos(&self) -> &Point {
        &self.pos
    }
}

impl fmt::Display for Bloc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self.kind {
            BlocKind::Air => ' ',
            BlocKind::Wall => '#',
            BlocKind::Coin => '.',
        };
        write!(f, "{}", if self.state { c } else { ' ' })
    }
}

#[derive(Debug, PartialEq)]
pub enum BlocKind {
    Air,
    Wall,
    Coin,
}
