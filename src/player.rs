use crate::Point;

const SPAWN_POINT: Point = Point(1, 1);

#[derive(Debug)]
pub struct Player {
    nick: String,
    pos: Point,
    score: usize,
}

impl Player {
    pub fn get_pts(&self) -> usize {
        self.score
    }

    pub fn get_pos(&self) -> &Point {
        &self.pos
    }

    pub fn set_pos(&mut self, new: Point) {
        self.pos = new;
    }
}

impl From<String> for Player {
    fn from(nick: String) -> Self {
        Self {
            nick,
            pos: SPAWN_POINT,
            score: 0,
        }
    }
}
