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
fn check_cell(chamber: &Vec<[bool; 7]>, cell: (i32, i8)) -> bool {
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
fn check_shape(chamber: &Vec<[bool; 7]>, shape: &Shape, pos: (i32, i8)) -> bool {
    if pos.0 < 0 {
        return true;
    }
    if pos.1 < 0 || pos.1 + shape.width as i8 - 1 >= 7 {
        return true;
    }

    return shape
        .cells
        .iter()
        .any(|c| check_cell(chamber, (pos.0 + c.0 as i32, pos.1 + c.1)));
}

// Fills in the shape onto the chamber at the specified position.
fn fill_shape(chamber: &mut Vec<[bool; 7]>, shape: &Shape, pos: (i32, i8)) {
    let max_row = pos.0 as usize + shape.height as usize - 1;

    if max_row >= chamber.len() {
        let num_rows = max_row - chamber.len() + 1;
        for _ in 0..num_rows {
            chamber.push([false; 7]);
        }
    }

    for c in &shape.cells {
        chamber[(pos.0 + c.0 as i32) as usize][(pos.1 + c.1) as usize] = true;
    }
}

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

    let mut chamber = Vec::new();
    let mut pattern_index = 0;

    for i in 0..2022 {
        let shape = &shapes[i % shapes.len()];
        let mut pos: (i32, i8) = (chamber.len() as i32 + 3, 2);

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

    println!("{}", chamber.len());
}
