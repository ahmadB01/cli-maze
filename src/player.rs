use crate::error::GameResult;
use crate::utils::unknown_name;
use crate::Point;
use std::io::{stdin, stdout, Write};

const SPAWN_POINT: Point = Point(1, 1);

fn ask_nick() -> GameResult<String> {
    print!(">>> Enter your nick: ");
    stdout().flush()?;

    let mut out = String::new();
    stdin().read_line(&mut out)?;

    if out.is_empty() || out.contains(' ') {
        Ok(unknown_name())
    } else {
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
        Ok(Self {
            nick: ask_nick()?,
            pos: spawn.unwrap_or(SPAWN_POINT),
            score: 0,
        })
    }

    pub fn with_nick(mut self, nick: String) -> Self {
        self.nick = nick;
        self
    }

    pub fn get_nick(&self) -> &String {
        &self.nick
    }

    pub fn plus1(&mut self) {
        self.score += 1
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
