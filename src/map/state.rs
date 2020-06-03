use crate::bloc::BlocKind;
use crate::error::GameResult;
use crate::map::utils::get_content;
use crate::map::Content;
use crate::map::State;
use crate::player::Player;
use crate::utils::unknown_name;
use crate::Point;

use crossterm::event::KeyCode;
use std::fmt;
use std::io::{Stdin, Stdout};
use std::path::Path;

const DEFAULT_RAD: usize = 2;

#[derive(Debug)]
pub struct Map {
    name: String,
    player: Player,
    pub(in crate::map) content: Content,
    coins: usize,
    state: State,
    pub(in crate::map) size: (usize, usize),
}

impl Map {
    pub fn new(path: &Path, stdin: &Stdin, stdout: &mut Stdout) -> GameResult<Self> {
        let (content, coins, spawn) = get_content(path)?;
        let (width, height) = (content[0].len(), content.len());
        let mut out = Self {
            name: unknown_name(),
            player: Player::new(spawn, stdin, stdout)?,
            content,
            coins,
            state: State::InGame,
            size: (width, height),
        };
        out.update(*out.player.get_pos());
        Ok(out)
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }

    pub fn f_state(self) -> State {
        self.state
    }

    pub fn get_content(&self) -> &Content {
        &self.content
    }

    pub fn move_p(&mut self, to: KeyCode) {
        let Point(x, y) = *self.player.get_pos();
        if self.is_overflow(Point(x, y), to) {
            return;
        }
        let pt = match to {
            KeyCode::Up => Point(x, y - 1),
            KeyCode::Right => Point(x + 1, y),
            KeyCode::Down => Point(x, y + 1),
            KeyCode::Left => Point(x - 1, y),
            _ => return,
        };
        if self.is_reachable(pt) {
            self.player.set_pos(pt);
            self.update(pt);
        }
    }

    fn finish(&mut self) {
        if self.player.get_pts() == self.coins {
            self.state = State::Win;
        } else {
            self.state = State::Loose;
        }
    }

    fn update(&mut self, pt: Point) {
        let current = self.at_pt(pt.0, pt.1);
        match current.get_type() {
            BlocKind::Coin => {
                let current = self.at_pt_mut(pt.0, pt.1);
                current.set_type(BlocKind::Air);
                self.player.plus1();
            }
            BlocKind::Output => self.finish(),
            _ => (),
        }
        let nb = self.neighbours(pt, DEFAULT_RAD);
        for line in self.content.iter_mut() {
            for bloc in line.iter_mut() {
                bloc.set_state(nb.contains(bloc.get_pos()));
            }
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = Vec::new();

        for y in 0..self.content.len() {
            let mut line = Vec::new();
            for x in 0..self.content[y].len() {
                line.push(
                    if x == self.player.get_pos().0 && y == self.player.get_pos().1 {
                        String::from("O")
                    } else {
                        self.at_pt(x, y).to_string()
                    },
                );
            }
            out.push(line.join(" "));
        }

        write!(
            f,
            "Map: {}\n\
             {}\n\
             Points to get: {}\n\
             {}'s points: {}",
            self.name,
            out.join("\n"),
            self.coins,
            self.player.get_nick(),
            self.player.get_pts()
        )
    }
}
