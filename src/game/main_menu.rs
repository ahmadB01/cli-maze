use crate::error::{GameError, GameResult};
use crate::game::mode::Mode;

use std::fmt;
use std::io::{Stdin, Stdout};

#[derive(Default)]
pub struct Menu<'a> {
    modes: Vec<&'a dyn Mode>,
}

impl<'a> Menu<'a> {
    pub fn new_mode(&mut self, mode: &'a dyn Mode) {
        self.modes.push(mode);
    }

    pub fn perform(&self, i: String, stdin: &Stdin, stdout: &mut Stdout) -> GameResult<()> {
        let i = i.trim().parse::<usize>()?;
        self.modes
            .get(i - 1)
            .ok_or(GameError::IncorrectInput)?
            .perform(stdin, stdout)?;
        Ok(())
    }
}

impl fmt::Display for Menu<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let modes = self
            .modes
            .iter()
            .enumerate()
            .map(|(i, mode)| format!("{} - {}\n", i + 1, mode.desc()))
            .collect::<String>();
        write!(f, "Please choose a mode by giving an index:\n{}", modes)
    }
}
