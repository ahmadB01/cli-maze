use crate::error::GameResult;
use crate::map::utils::append_data;
use crate::utils::unknown_name;
use crate::Point;

const SCORES_PATH: &str = "./scores";

use std::io::{Stdin, Stdout, Write};

const SPAWN_POINT: Point = Point(1, 1);

fn ask_nick(stdin: &Stdin, stdout: &mut Stdout) -> GameResult<String> {
    print!("Enter your nick: ");
    stdout.flush()?;

    let mut out = String::new();
    stdin.read_line(&mut out)?;

    if out.trim().is_empty() || out.contains(' ') {
        Ok(unknown_name())
    } else {
        Ok(out.trim().to_owned())
    }
}

#[derive(Debug)]
pub struct Player {
    nick: String,
    pos: Point,
    score: usize,
}

impl Player {
    pub fn new(spawn: Option<Point>, stdin: &Stdin, stdout: &mut Stdout) -> GameResult<Self> {
        Ok(Self {
            nick: ask_nick(stdin, stdout)?,
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

impl Drop for Player {
    fn drop(&mut self) {
        let raw = format!("{}={};", self.nick, self.score);
        append_data(raw, SCORES_PATH).unwrap();
    }
}
