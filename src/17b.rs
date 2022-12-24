use std::collections::HashMap;
use std::io;
use std::vec::Vec;

enum Direction {
    Left,
    Right,
}

struct Shape {
    // width and height are used for positioning calculations/checks. Make sure
    // they correspond with the coordinates in cells.
    width: u8,
    height: u8,
    // cells are which cells the shape occupy, in (row, column) format, where
    // (0, 0) represents the bottom-left corner of the shape and increasing
    // coordinates move towards the top-right.
    cells: Vec<(i8, i8)>,
}

// Returns whether the cell in the chamber is occupied/blocked.
fn check_cell(chamber: &Vec<[bool; 7]>, cell: (i64, i8)) -> bool {
    if cell.0 < 0 {
        return true;
    }
    if cell.1 < 0 || cell.1 >= 7 {
        return true;
    }

    if cell.0 as usize >= chamber.len() {
        return false;
    }

    return chamber[cell.0 as usize][cell.1 as usize];
}

// Returns whether the cells in the specified shape are occupied/blocked.
fn check_shape(chamber: &Vec<[bool; 7]>, shape: &Shape, pos: (i64, i8)) -> bool {
    if pos.0 < 0 {
        return true;
    }
    if pos.1 < 0 || pos.1 + shape.width as i8 - 1 >= 7 {
        return true;
    }

    return shape
        .cells
        .iter()
        .any(|c| check_cell(chamber, (pos.0 + c.0 as i64, pos.1 + c.1)));
}

// Fills in the shape onto the chamber at the specified position.
fn fill_shape(chamber: &mut Vec<[bool; 7]>, shape: &Shape, pos: (i64, i8)) {
    let max_row = pos.0 as usize + shape.height as usize - 1;

    if max_row >= chamber.len() {
        let num_rows = max_row - chamber.len() + 1;
        for _ in 0..num_rows {
            chamber.push([false; 7]);
        }
    }

    for c in &shape.cells {
        chamber[(pos.0 + c.0 as i64) as usize][(pos.1 + c.1) as usize] = true;
    }
}

fn get_row(chamber: &Vec<[bool; 7]>, row: i64) -> [bool; 7] {
    if row < 0 {
        return [true; 7];
    }
    return chamber[row as usize];
}

fn get_top(chamber: &Vec<[bool; 7]>) -> [[bool; 7]; 6] {
    let len = chamber.len() as i64;

    return (1..7)
        .map(|i| get_row(chamber, len - i))
        .collect::<Vec<[bool; 7]>>()
        .try_into()
        .unwrap();
}

const ITERATIONS: usize = 1000000000000;

fn main() {
    let shapes = [
        // -
        Shape {
            width: 4,
            height: 1,
            cells: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        },
        // +
        Shape {
            width: 3,
            height: 3,
            cells: vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
        },
        // _|
        Shape {
            width: 3,
            height: 3,
            cells: vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
        },
        // |
        Shape {
            width: 1,
            height: 4,
            cells: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        },
        // (square)
        Shape {
            width: 2,
            height: 2,
            cells: vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        },
    ];

    let mut input = io::stdin().lines();

    let pattern: Vec<Direction> = input
        .next()
        .expect("input should have one line")
        .unwrap()
        .chars()
        .map(|c| match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("direction should be < or >"),
        })
        .collect();

    // Assumption: the shapes will fall into the chamber in a cycle, which we
    // should be able to detect by when shape index and pattern index are both 0
    // again. Therefore, we just need to find when we hit the beginning of the
    // cycle again and then compute the height of the chamber based on how many
    // cycles we go through and which iteration we end on.
    let mut chamber = Vec::new();
    let mut pattern_index = 0;
    // heights stores the height of the chamber on iteration i.
    let mut heights = Vec::new();
    // cycle_map stores (shape index, pattern_index, top) to iteration index.
    let mut cycle_map = HashMap::new();

    for i in 0..ITERATIONS {
        let key = (i % shapes.len(), pattern_index, get_top(&chamber));
        if cycle_map.contains_key(&key) {
            break;
        }
        cycle_map.insert(key, i);
        heights.push(chamber.len());

        let shape = &shapes[i % shapes.len()];
        let mut pos: (i64, i8) = (chamber.len() as i64 + 3, 2);

        'fall_loop: loop {
            // Try jet push left/right 1 unit.
            let jet_pos = (
                pos.0,
                pos.1
                    + match pattern[pattern_index] {
                        Direction::Left => -1,
                        Direction::Right => 1,
                    },
            );
            pattern_index = (pattern_index + 1) % pattern.len();
            if !check_shape(&chamber, shape, jet_pos) {
                pos = jet_pos;
            }

            // Try falling down 1 unit.
            let fall_pos = (pos.0 - 1, pos.1);
            if check_shape(&chamber, shape, fall_pos) {
                break 'fall_loop;
            } else {
                pos = fall_pos;
            }
        }

        fill_shape(&mut chamber, shape, pos);
    }

    let cycle_end = heights.len();
    let cycle_start = *cycle_map
        .get(&(cycle_end % shapes.len(), pattern_index, get_top(&chamber)))
        .unwrap();
    let cycle_height = chamber.len() - heights[cycle_start];
    let cycle_len = cycle_end - cycle_start;
    let num_cycles = (ITERATIONS - cycle_start) / cycle_len;

    let height =
        num_cycles * cycle_height + heights[cycle_start + (ITERATIONS - cycle_start) % cycle_len];

    println!("{}", height);
}
