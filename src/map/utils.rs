use crate::bloc::{Bloc, BlocKind};
use crate::error::{GameError, GameResult};
use crate::map::Content;
use crate::map::Map;
use crate::utils::read_file;
use crate::Point;
use std::path::Path;

fn get_raw(txt: String) -> String {
    txt.lines()
        .map(|line| line.chars().into_iter().step_by(2).collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn get_content(path: &Path) -> GameResult<(Content, usize)> {
    let mut coins = 0usize;
    let txt = match read_file(path) {
        Ok(content) => content,
        Err(e) => return Err(GameError::IoError(Some(path.to_path_buf()), e)),
    };
    let raw = get_raw(txt);
    let mut out = Vec::new();
    for (y, l) in raw.lines().enumerate() {
        let mut line = Vec::new();
        for (x, c) in l.chars().enumerate() {
            let kind = match c {
                '#' => BlocKind::Wall,
                ' ' => BlocKind::Air,
                '.' => {
                    coins += 1;
                    BlocKind::Coin
                }
                _ => return Err(GameError::InvalidMapFile(path.to_path_buf(), c)),
            };
            let bloc = Bloc::new(Point(x, y), kind);
            line.push(bloc);
        }
        out.push(line);
    }

    Ok((out, coins))
}

fn neighbours(tgt: Point) -> [Point; 8] {
    let Point(x, y) = tgt;
    [
        Point(x - 1, y - 1),
        Point(x, y - 1),
        Point(x + 1, y - 1),
        Point(x + 1, y),
        Point(x + 1, y + 1),
        Point(x, y + 1),
        Point(x - 1, y + 1),
        Point(x - 1, y),
    ]
}

impl Map {
    pub(in crate::map) fn at_borders(&self, x: usize, y: usize) -> bool {
        y == 0 || y == self.content.len() || x == 0 || x == self.content[0].len()
    }

    pub(in crate::map) fn reachable(&self, pt: Point) -> bool {
        let Point(x, y) = pt;
        *self.content[y][x].get_type() != BlocKind::Wall
    }

    pub(in crate::map) fn update(&mut self, pt: Point) {
        let nb = neighbours(pt);
        for line in self.content.iter_mut() {
            for bloc in line.iter_mut() {
                bloc.set_state(nb.contains(bloc.get_pos()));
            }
        }
    }
}