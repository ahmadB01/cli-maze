use crate::bloc::{Bloc, BlocKind};
use crate::error::{GameError, GameResult};
use crate::map::{Content, Map};
use crate::utils::read_file;
use crate::Point;
use crossterm::event::KeyCode;
use std::path::Path;

const DEFAULT_RAD: usize = 2;

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
                '#' | 'i' => BlocKind::Wall,
                ' ' => BlocKind::Air,
                '.' => {
                    coins += 1;
                    BlocKind::Coin
                }
                'o' => BlocKind::Output,
                _ => return Err(GameError::InvalidMapFile(path.to_path_buf(), c)),
            };

            let bloc = Bloc::new(Point(x, y), kind);
            line.push(bloc);
        }

        out.push(line);
    }

    Ok((out, coins))
}

impl Map {
    pub(in crate::map) fn is_reachable(&self, pt: Point) -> bool {
        *self.at_pt(pt).get_type() != BlocKind::Wall
    }

    pub(in crate::map) fn update(&mut self, pt: Point) {
        let nb = self.neighbours(pt, DEFAULT_RAD);
        for line in self.content.iter_mut() {
            for bloc in line.iter_mut() {
                bloc.set_state(nb.contains(bloc.get_pos()));
            }
        }
    }

    pub(in crate::map) fn neighbours(&self, tgt: Point, rad: usize) -> Vec<Point> {
        let width = self.content[0].len();
        let height = self.content.len();

        let Point(x, y) = tgt;
        let mut out = Vec::new();

        let from_y = if rad > y { 0 } else { y - rad };
        let to_y = if y + rad + 1 >= height {
            height
        } else {
            y + rad + 1
        };
        let from_x = if rad > x { 0 } else { x - rad };
        let to_x = if x + rad + 1 >= width {
            width
        } else {
            x + rad + 1
        };

        for j in from_y..to_y {
            for i in from_x..to_x {
                out.push(Point(i, j));
            }
        }

        out
    }

    pub(in crate::map) fn at_pt(&self, pt: Point) -> &Bloc {
        &self.content[pt.1][pt.0]
    }

    pub(in crate::map) fn is_overflow(&self, pt: Point, to: &KeyCode) -> bool {
        // i must make a `size` field to my Map struct
        let max_x = self.content[0].len() - 1;
        let max_y = self.content.len() - 1;

        // didnt find another way to make that
        match (pt, to) {
            (Point(0, _), KeyCode::Left) | (Point(_, 0), KeyCode::Up) => true,
            (Point(x, _), KeyCode::Right) if x == max_x => true,
            (Point(_, y), KeyCode::Down) if y == max_y => true,
            _ => false,
        }
    }
}
