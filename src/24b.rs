use std::collections::HashSet;
use std::io;
use std::vec::Vec;

#[derive(Clone)]
struct Cell {
    north: bool,
    east: bool,
    south: bool,
    west: bool,
}

impl Cell {
    fn is_free(&self) -> bool {
        !(self.north || self.east || self.south || self.west)
    }
}

fn trip(blizzard_map: &mut Vec<Vec<Cell>>, start: (usize, usize), end: (usize, usize)) -> u16 {
    let mut states = HashSet::from([start]);
    let mut step_count = 0;

    let num_rows = blizzard_map.len();
    let num_cols = blizzard_map[0].len();

    'search_loop: while !states.is_empty() {
        step_count += 1;

        // Update blizzard map.
        let old_map: Vec<Vec<Cell>> = blizzard_map.iter().map(|v| v.clone()).collect();
        for (i, row) in blizzard_map.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                cell.north = old_map[(i + 1) % num_rows][j].north;
                cell.east = old_map[i][(j + num_cols - 1) % num_cols].east;
                cell.south = old_map[(i + num_rows - 1) % num_rows][j].south;
                cell.west = old_map[i][(j + 1) % num_cols].west;
            }
        }

        // Expand out set of search states for next step.
        let mut next_states = HashSet::new();
        'expand_loop: for state in states.drain() {
            if state == start {
                next_states.insert(start);
                let enter_state = (
                    if start.0 < end.0 {
                        start.0 + 1
                    } else {
                        start.0 - 1
                    },
                    start.1,
                );
                if blizzard_map[enter_state.0 - 1][enter_state.1 - 1].is_free() {
                    next_states.insert(enter_state);
                }
                continue 'expand_loop;
            }

            let test_states = [
                (state.0, state.1),
                (state.0 + 1, state.1),
                (state.0 - 1, state.1),
                (state.0, state.1 + 1),
                (state.0, state.1 - 1),
            ];
            for test_state in test_states {
                if test_state == end {
                    break 'search_loop;
                }

                if test_state.0 < 1
                    || test_state.0 > num_rows
                    || test_state.1 < 1
                    || test_state.1 > num_cols
                {
                    continue;
                }

                if blizzard_map[test_state.0 - 1][test_state.1 - 1].is_free() {
                    next_states.insert(test_state);
                }
            }
        }

        states = next_states;
    }

    return step_count;
}

fn main() {
    let mut lines = io::stdin().lines();
    let mut blizzard_map = Vec::new();

    // First line contains start position.
    let start = (
        0,
        lines
            .next()
            .unwrap()
            .unwrap()
            .find('.')
            .expect("first line should have non-wall opening"),
    );
    let mut end = (0, 0);

    for (i, line) in lines.enumerate() {
        let line = line.unwrap();
        if line.chars().nth(1).expect("line should have index 1") == '#' {
            // Valley should be a rectangle of ground surrounded by walls, so a
            // wall at index 1 should mean we hit the bottom of the valley.
            end = (
                i + 1,
                line.find('.')
                    .expect("last line should have non-wall opening"),
            );
            break;
        }

        let row: Vec<Cell> = line[1..(line.len() - 1)]
            .chars()
            .map(|c| Cell {
                north: c == '^',
                east: c == '>',
                south: c == 'v',
                west: c == '<',
            })
            .collect();
        blizzard_map.push(row);
    }

    let mut steps = 0;
    steps += trip(&mut blizzard_map, start, end);
    steps += trip(&mut blizzard_map, end, start);
    steps += trip(&mut blizzard_map, start, end);

    println!("{}", steps);
}
