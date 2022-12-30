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

#[derive(Clone)]
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

#[derive(Clone)]
struct State {
    pos: (usize, usize),
    dir: FacingDirection,
}

fn edge_state(
    tile: (usize, usize),
    tile_size: usize,
    offset: usize,
    dir: FacingDirection,
    flip: bool,
) -> State {
    let base_pos = (tile.0 * tile_size + 1, tile.1 * tile_size + 1);
    let opposite_end = tile_size - 1;
    let tile_offset = if flip { opposite_end - offset } else { offset };

    return State {
        pos: match dir {
            FacingDirection::Right => (base_pos.0 + tile_offset, base_pos.1),
            FacingDirection::Down => (base_pos.0, base_pos.1 + tile_offset),
            FacingDirection::Left => (base_pos.0 + tile_offset, base_pos.1 + opposite_end),
            FacingDirection::Up => (base_pos.0 + opposite_end, base_pos.1 + tile_offset),
        },
        dir: dir,
    };
}

fn get_states_to_check(state: &State, tile_size: usize) -> Vec<State> {
    let base_tile = ((state.pos.0 - 1) / tile_size, (state.pos.1 - 1) / tile_size);

    let mut res = Vec::new();

    // Note: this is probably not comprehensive of all possible cube net cases,
    // but this should be enough to cover the sample input + the actual input.
    match state.dir {
        FacingDirection::Right => {
            let offset = (state.pos.0 - 1) % tile_size;

            // Up 1, Right 1
            if base_tile.0 > 0 {
                res.push(edge_state(
                    (base_tile.0 - 1, base_tile.1 + 1),
                    tile_size,
                    offset,
                    FacingDirection::Up,
                    false,
                ));
            }
            // Down 1, Right 1
            res.push(edge_state(
                (base_tile.0 + 1, base_tile.1 + 1),
                tile_size,
                offset,
                FacingDirection::Down,
                true,
            ));
            // Up 2, Left 1
            if base_tile.0 > 1 && base_tile.1 > 0 {
                res.push(edge_state(
                    (base_tile.0 - 2, base_tile.1 - 1),
                    tile_size,
                    offset,
                    FacingDirection::Left,
                    true,
                ));
            }
            // Up 2, Right 1
            if base_tile.0 > 1 {
                res.push(edge_state(
                    (base_tile.0 - 2, base_tile.1 + 1),
                    tile_size,
                    offset,
                    FacingDirection::Left,
                    true,
                ));
            }
            // Down 2, Right 1
            res.push(edge_state(
                (base_tile.0 + 2, base_tile.1 + 1),
                tile_size,
                offset,
                FacingDirection::Left,
                true,
            ));
            // Down 2, Left 1
            if base_tile.1 > 0 {
                res.push(edge_state(
                    (base_tile.0 + 2, base_tile.1 - 1),
                    tile_size,
                    offset,
                    FacingDirection::Left,
                    true,
                ));
            }
        }
        FacingDirection::Left => {
            let offset = (state.pos.0 - 1) % tile_size;

            // Up 1, Left 1
            if base_tile.0 > 0 && base_tile.1 > 0 {
                res.push(edge_state(
                    (base_tile.0 - 1, base_tile.1 - 1),
                    tile_size,
                    offset,
                    FacingDirection::Up,
                    true,
                ));
            }
            // Down 1, Left 1
            if base_tile.1 > 0 {
                res.push(edge_state(
                    (base_tile.0 + 1, base_tile.1 - 1),
                    tile_size,
                    offset,
                    FacingDirection::Down,
                    false,
                ));
            }
            // Down 1, Right 3
            res.push(edge_state(
                (base_tile.0 + 1, base_tile.1 + 3),
                tile_size,
                offset,
                FacingDirection::Up,
                true,
            ));
            // Up 3, Right 1
            if base_tile.0 > 2 {
                res.push(edge_state(
                    (base_tile.0 - 3, base_tile.1 + 1),
                    tile_size,
                    offset,
                    FacingDirection::Down,
                    false,
                ));
            }
            // Down 2, Left 1
            if base_tile.1 > 0 {
                res.push(edge_state(
                    (base_tile.0 + 2, base_tile.1 - 1),
                    tile_size,
                    offset,
                    FacingDirection::Right,
                    true,
                ));
            }
            // Up 2, Right 1
            if base_tile.0 > 1 {
                res.push(edge_state(
                    (base_tile.0 - 2, base_tile.1 + 1),
                    tile_size,
                    offset,
                    FacingDirection::Right,
                    true,
                ));
            }
        }
        FacingDirection::Down => {
            let offset = (state.pos.1 - 1) % tile_size;

            // Down 1, Left 1
            if base_tile.1 > 0 {
                res.push(edge_state(
                    (base_tile.0 + 1, base_tile.1 - 1),
                    tile_size,
                    offset,
                    FacingDirection::Left,
                    false,
                ));
            }
            // Down 1, Right 1
            res.push(edge_state(
                (base_tile.0 + 1, base_tile.1 + 1),
                tile_size,
                offset,
                FacingDirection::Right,
                true,
            ));
            // Up 1, Left 2
            if base_tile.0 > 0 && base_tile.1 > 1 {
                res.push(edge_state(
                    (base_tile.0 - 1, base_tile.1 - 2),
                    tile_size,
                    offset,
                    FacingDirection::Up,
                    true,
                ));
            }
            // Up 1, Left 3
            if base_tile.0 > 0 && base_tile.1 > 2 {
                res.push(edge_state(
                    (base_tile.0 - 1, base_tile.1 - 3),
                    tile_size,
                    offset,
                    FacingDirection::Right,
                    true,
                ));
            }
            // Up 3, Right 2
            if base_tile.0 > 2 {
                res.push(edge_state(
                    (base_tile.0 - 3, base_tile.1 + 2),
                    tile_size,
                    offset,
                    FacingDirection::Down,
                    false,
                ));
            }
        }
        FacingDirection::Up => {
            let offset = (state.pos.1 - 1) % tile_size;

            // Up 1, Left 1
            if base_tile.0 > 0 && base_tile.1 > 0 {
                res.push(edge_state(
                    (base_tile.0 - 1, base_tile.1 - 1),
                    tile_size,
                    offset,
                    FacingDirection::Left,
                    true,
                ));
            }
            // Up 1, Right 1
            if base_tile.0 > 0 {
                res.push(edge_state(
                    (base_tile.0 - 1, base_tile.1 + 1),
                    tile_size,
                    offset,
                    FacingDirection::Right,
                    false,
                ));
            }
            // Up 1, Right 2
            if base_tile.0 > 0 {
                res.push(edge_state(
                    (base_tile.0 - 1, base_tile.1 + 2),
                    tile_size,
                    offset,
                    FacingDirection::Down,
                    true,
                ));
            }
            // Down 1, Left 2
            if base_tile.1 > 1 {
                res.push(edge_state(
                    (base_tile.0 + 1, base_tile.1 - 2),
                    tile_size,
                    offset,
                    FacingDirection::Down,
                    true,
                ));
            }
            // Down 3, Left 1
            if base_tile.1 > 0 {
                res.push(edge_state(
                    (base_tile.0 + 3, base_tile.1 - 1),
                    tile_size,
                    offset,
                    FacingDirection::Right,
                    false,
                ));
            }
            // Down 3, Left 2
            if base_tile.1 > 1 {
                res.push(edge_state(
                    (base_tile.0 + 3, base_tile.1 - 2),
                    tile_size,
                    offset,
                    FacingDirection::Up,
                    false,
                ));
            }
        }
    }

    return res;
}

fn main() {
    let mut map = HashMap::new();
    let mut state = State {
        pos: (0, 0),
        dir: FacingDirection::Right,
    };

    for (i, line) in io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .take_while(|l| !l.is_empty())
        .enumerate()
    {
        for (j, c) in line.chars().enumerate() {
            let coord = (i + 1, j + 1);
            match c {
                '.' => {
                    map.insert(coord, true);
                    if state.pos.0 == 0 {
                        state.pos = coord;
                    }
                }
                '#' => {
                    map.insert(coord, false);
                    if state.pos.0 == 0 {
                        state.pos = coord;
                    }
                }
                ' ' => {}
                _ => panic!("unexpected character in map"),
            }
        }
    }
    let tile_size = ((map.len() / 6) as f32).sqrt().round() as usize;

    let mut path = String::new();
    let _ = io::stdin().read_line(&mut path);
    let path = parse_path(&path);

    for p in path {
        match p {
            Move::Forward(s) => {
                'forward_loop: for _ in 0..s {
                    let delta = state.dir.delta();
                    let mut new_state = State {
                        pos: (
                            (state.pos.0 as isize + delta.0) as usize,
                            (state.pos.1 as isize + delta.1) as usize,
                        ),
                        dir: state.dir.clone(),
                    };

                    if !map.contains_key(&new_state.pos) {
                        new_state = get_states_to_check(&state, tile_size)
                            .iter()
                            .filter(|s| map.contains_key(&s.pos))
                            .next()
                            .expect("new pos should have been found")
                            .clone();
                    }

                    if *map
                        .get(&new_state.pos)
                        .expect("map should have cell for new pos")
                    {
                        state = new_state;
                    } else {
                        break 'forward_loop;
                    }
                }
            }
            Move::Turn(d) => {
                state.dir = state.dir.turn(d);
            }
        }
    }

    let password = state.pos.0 * 1000 + 4 * state.pos.1 + state.dir.as_num();
    println!("{}", password);
}
