use std::io;
use std::vec::Vec;

fn main() {
    // Parse input into matrix (vec of vec) of digits.
    let map = io::stdin()
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).expect("map should contain digits"))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    // Scan map and mark visible trees.
    let rows = map.len();
    let cols = map[0].len();

    let mut vis_map = Vec::new();
    vis_map.resize(rows, {
        let mut vis_row = Vec::new();
        vis_row.resize(cols, false);
        vis_row
    });

    for r in 0..rows {
        let mut max = None;
        for c in 0..cols - 1 {
            if max.map_or(true, |v| map[r][c] > v) {
                vis_map[r][c] = true;
                max = Some(map[r][c]);
            }
        }
        max = None;
        for c in (1..cols).rev() {
            if max.map_or(true, |v| map[r][c] > v) {
                vis_map[r][c] = true;
                max = Some(map[r][c]);
            }
        }
    }

    for c in 0..cols {
        let mut max = None;
        for r in 0..rows - 1 {
            if max.map_or(true, |v| map[r][c] > v) {
                vis_map[r][c] = true;
                max = Some(map[r][c]);
            }
        }
        max = None;
        for r in (1..rows).rev() {
            if max.map_or(true, |v| map[r][c] > v) {
                vis_map[r][c] = true;
                max = Some(map[r][c]);
            }
        }
    }

    let sum: usize = vis_map
        .iter()
        .map(|r| r.iter().filter(|&v| *v).count())
        .sum();
    println!("{}", sum);
}
