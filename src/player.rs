use crate::error::GameResult;
use crate::utils::unknown_name;
use crate::Point;

const SPAWN_POINT: Point = Point(1, 1);

fn ask_nick() -> GameResult<String> {
    use std::io::{stdin, stdout, Write};

    print!(">>> Enter your nick: ");
    stdout().flush()?;

    let mut out = String::new();
    stdin().read_line(&mut out)?;

    if out.is_empty() || out.contains(' ') {
        println!("Come on that's not funny.");
        Ok(unknown_name())
    } else {
        println!("Ok {}.", out);
        Ok(out)
    }
}

#[derive(Debug)]
pub struct Player {
    nick: String,
    pos: Point,
    score: usize,
}

impl Player {
    pub fn at(spawn: Option<Point>) -> GameResult<Self> {
        let spawn = match spawn {
            Some(p) => p,
            None => SPAWN_POINT,
        };
        Ok(Self {
            nick: ask_nick()?,
            pos: spawn,
            score: 0,
        })
    }

    pub fn with_nick(mut self, nick: String) -> Self {
        self.nick = nick;
        self
    }

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
