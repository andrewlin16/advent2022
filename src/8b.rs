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

    let rows = map.len();
    let cols = map[0].len();

    let mut max = 0;

    for r in 1..rows - 1 {
        for c in 1..cols - 1 {
            let height = map[r][c];
            let mut score = 1;

            let mut dist = 1;
            'east_loop: for cc in c + 1..cols - 1 {
                if map[r][cc] >= height {
                    break 'east_loop;
                }
                dist += 1;
            }
            score *= dist;

            dist = 1;
            'west_loop: for cc in (1..c).rev() {
                if map[r][cc] >= height {
                    break 'west_loop;
                }
                dist += 1;
            }
            score *= dist;

            dist = 1;
            'south_loop: for rr in r + 1..rows - 1 {
                if map[rr][c] >= height {
                    break 'south_loop;
                }
                dist += 1;
            }
            score *= dist;

            dist = 1;
            'north_loop: for rr in (1..r).rev() {
                if map[rr][c] >= height {
                    break 'north_loop;
                }
                dist += 1;
            }
            score *= dist;

            if score > max {
                max = score;
            }
        }
    }

    println!("{}", max);
}
