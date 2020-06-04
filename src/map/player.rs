use crate::map::utils::append_data;
use crate::utils::ask_name;
use crate::GameResult;
use crate::Point;

const SCORES_PATH: &str = "./scores";

use std::io::{Stdin, Stdout};

const SPAWN_POINT: Point = Point(1, 1);

#[derive(Debug)]
pub struct Player {
    nick: String,
    pos: Point,
    score: usize,
}

impl Player {
    pub fn new(spawn: Option<Point>, stdin: &Stdin, stdout: &mut Stdout) -> GameResult<Self> {
        Ok(Self {
            nick: ask_name("Enter your nick: ", stdin, stdout)?,
            pos: spawn.unwrap_or(SPAWN_POINT),
            score: 0,
        })
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
