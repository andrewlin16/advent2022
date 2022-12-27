use std::cmp;
use std::collections::HashMap;
use std::io;
use std::vec::Vec;

enum Move {
    Forward(u8),
    Turn(TurnDirection),
}

enum TurnDirection {
    Right,
    Left,
}

impl TurnDirection {
    fn from(c: char) -> Option<TurnDirection> {
        match c {
            'R' => Some(TurnDirection::Right),
            'L' => Some(TurnDirection::Left),
            _ => None,
        }
    }
}

enum FacingDirection {
    Right,
    Down,
    Left,
    Up,
}

impl FacingDirection {
    fn turn(&self, dir: TurnDirection) -> FacingDirection {
        match dir {
            TurnDirection::Right => match self {
                FacingDirection::Right => FacingDirection::Down,
                FacingDirection::Down => FacingDirection::Left,
                FacingDirection::Left => FacingDirection::Up,
                FacingDirection::Up => FacingDirection::Right,
            },
            TurnDirection::Left => match self {
                FacingDirection::Right => FacingDirection::Up,
                FacingDirection::Down => FacingDirection::Right,
                FacingDirection::Left => FacingDirection::Down,
                FacingDirection::Up => FacingDirection::Left,
            },
        }
    }

    fn as_num(&self) -> usize {
        match self {
            FacingDirection::Right => 0,
            FacingDirection::Down => 1,
            FacingDirection::Left => 2,
            FacingDirection::Up => 3,
        }
    }

    fn delta(&self) -> (isize, isize) {
        match self {
            FacingDirection::Right => (0, 1),
            FacingDirection::Down => (1, 0),
            FacingDirection::Left => (0, -1),
            FacingDirection::Up => (-1, 0),
        }
    }
}

fn update_maps(
    map: &mut HashMap<(usize, usize), bool>,
    row_bounds: &mut HashMap<usize, (usize, usize)>,
    col_bounds: &mut HashMap<usize, (usize, usize)>,
    coord: (usize, usize),
    state: bool,
) {
    map.insert(coord, state);

    match row_bounds.get(&coord.0) {
        Some(&(min, max)) => {
            row_bounds.insert(coord.0, (cmp::min(min, coord.1), cmp::max(max, coord.1)));
        }
        None => {
            row_bounds.insert(coord.0, (coord.1, coord.1));
        }
    };

    match col_bounds.get(&coord.1) {
        Some(&(min, max)) => {
            col_bounds.insert(coord.1, (cmp::min(min, coord.0), cmp::max(max, coord.0)));
        }
        None => {
            col_bounds.insert(coord.1, (coord.0, coord.0));
        }
    };
}

fn parse_path(s: &String) -> Vec<Move> {
    let mut path = Vec::new();
    let mut steps = 0;

    for c in s.chars() {
        if let Some(turn) = TurnDirection::from(c) {
            if steps > 0 {
                path.push(Move::Forward(steps));
                steps = 0;
            }
            path.push(Move::Turn(turn));
        } else if let Some(digit) = c.to_digit(10) {
            steps = steps * 10 + digit as u8;
        } else {
            // Assume end of path.
            break;
        }
    }

    if steps > 0 {
        path.push(Move::Forward(steps));
    }

    return path;
}

fn main() {
    let mut map = HashMap::new();
    let mut row_bounds = HashMap::new();
    let mut col_bounds = HashMap::new();

    for (i, line) in io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .take_while(|l| !l.is_empty())
        .enumerate()
    {
        for (j, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    update_maps(
                        &mut map,
                        &mut row_bounds,
                        &mut col_bounds,
                        (i + 1, j + 1),
                        true,
                    );
                }
                '#' => {
                    update_maps(
                        &mut map,
                        &mut row_bounds,
                        &mut col_bounds,
                        (i + 1, j + 1),
                        false,
                    );
                }
                ' ' => {}
                _ => panic!("unexpected character in map"),
            }
        }
    }

    let mut path = String::new();
    let _ = io::stdin().read_line(&mut path);
    let path = parse_path(&path);

    // Trace through path.
    let mut pos = (
        1,
        col_bounds.get(&1).expect("bound should exist for row 1").0,
    );
    let mut dir = FacingDirection::Right;

    for p in path {
        match p {
            Move::Forward(s) => {
                for _ in 0..s {
                    let delta = dir.delta();
                    let mut new_pos = (
                        (pos.0 as isize + delta.0) as usize,
                        (pos.1 as isize + delta.1) as usize,
                    );
                    if !map.contains_key(&new_pos) {
                        new_pos = match dir {
                            FacingDirection::Right => (
                                pos.0,
                                row_bounds
                                    .get(&pos.0)
                                    .expect("bounds should exist for row")
                                    .0,
                            ),
                            FacingDirection::Left => (
                                pos.0,
                                row_bounds
                                    .get(&pos.0)
                                    .expect("bounds should exist for row")
                                    .1,
                            ),
                            FacingDirection::Down => (
                                col_bounds
                                    .get(&pos.1)
                                    .expect("bounds should exist for col")
                                    .0,
                                pos.1,
                            ),
                            FacingDirection::Up => (
                                col_bounds
                                    .get(&pos.1)
                                    .expect("bounds should exist for col")
                                    .1,
                                pos.1,
                            ),
                        };
                    }
                    if *map.get(&new_pos).expect("map should have cell for pos") {
                        pos = new_pos;
                    }
                }
            }
            Move::Turn(d) => {
                dir = dir.turn(d);
            }
        }
    }

    // Calculate password.
    let password = pos.0 * 1000 + 4 * pos.1 + dir.as_num();
    println!("{}", password);
}
