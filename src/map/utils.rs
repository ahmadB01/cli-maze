use crate::error::{GameError, GameResult};
use crate::map::bloc::{Bloc, BlocKind};
use crate::map::{Content, Map};
use crate::utils::read_file;
use crate::Point;
use crossterm::event::KeyCode;
use std::path::Path;

fn get_raw(txt: String) -> String {
    txt.lines()
        .map(|line| line.chars().step_by(2).collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

// TODO: shrink this function
pub fn get_content(path: &Path) -> GameResult<(Content, usize, Option<Point>)> {
    let mut coins = 0usize;

    let txt = match read_file(path) {
        Ok(content) => content,
        Err(e) => return Err(GameError::IoError(Some(path.to_path_buf()), e)),
    };

    let raw = get_raw(txt);
    let mut out = Vec::new();

    let mut spawn = Option::<Point>::None;

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
                'i' => {
                    spawn = Some(Point(x, y));
                    BlocKind::Wall
                }
                'o' => BlocKind::Output,
                _ => return Err(GameError::InvalidMapFile(path.to_path_buf(), c)),
            };

            let bloc = Bloc::new(Point(x, y), kind);
            line.push(bloc);
        }

        out.push(line);
    }

    Ok((out, coins, spawn))
}

impl Map {
    pub(in crate::map) fn is_reachable(&self, pt: Point) -> bool {
        let Point(x, y) = pt;
        *self.at_pt(x, y).get_type() != BlocKind::Wall
    }

    pub(in crate::map) fn neighbours(&self, tgt: Point, rad: usize) -> Vec<Point> {
        let (width, height) = self.size;

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

    pub(in crate::map) fn at_pt(&self, x: usize, y: usize) -> &Bloc {
        &self.content[y][x]
    }

    pub(in crate::map) fn at_pt_mut(&mut self, x: usize, y: usize) -> &mut Bloc {
        self.content
            .get_mut(y)
            .and_then(|line| line.get_mut(x))
            .unwrap()
    }

    pub(in crate::map) fn is_overflow(&self, pt: Point, to: KeyCode) -> bool {
        let max_x = self.size.0 - 1;
        let max_y = self.size.1 - 1;

        // didnt find another way to make that
        match (pt, to) {
            (Point(0, _), KeyCode::Left) | (Point(_, 0), KeyCode::Up) => true,
            (Point(x, _), KeyCode::Right) if x == max_x => true,
            (Point(_, y), KeyCode::Down) if y == max_y => true,
            _ => false,
        }
    }
}
