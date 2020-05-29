use crate::error::GameResult;
use crate::map::utils::get_content;
use crate::map::State;
use crate::Point;
use crossterm::event::KeyCode;
use std::fmt;
use std::path::Path;
use uuid::Uuid;

use crate::map::Content;
use crate::player::Player;

#[derive(Debug)]
pub struct Map {
    name: String,
    player: Player,
    pub(in crate::map) content: Content,
    coins: usize,
    state: State,
}

impl Map {
    pub fn new(player: Player, path: &Path) -> GameResult<Self> {
        let (content, coins) = get_content(path)?;
        let id = Uuid::new_v4();
        let init_name = format!("unknown: {}", id);
        let spawn = player.get_pos().clone();
        let mut out = Self {
            name: init_name,
            player,
            content,
            coins,
            state: State::InGame,
        };
        out.update(spawn);
        Ok(out)
    }

    pub fn with_name(&mut self, name: String) -> &Self {
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
        let Point(x, y) = self.player.get_pos().clone();
        if self.is_overflow(Point(x, y), &to) {
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
                        self.content[y][x].to_string()
                    },
                );
            }
            out.push(line.join(" "));
        }

        write!(f, "{}", out.join("\n"))
    }
}
