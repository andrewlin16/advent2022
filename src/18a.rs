use std::collections::HashSet;
use std::io;

fn main() {
    let points: HashSet<(i8, i8, i8)> = io::stdin()
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let mut s = line.splitn(3, ",");
            let x = s
                .next()
                .expect("line should have x coordinate")
                .parse()
                .expect("x coordinate should be numeric");
            let y = s
                .next()
                .expect("line should have y coordinate")
                .parse()
                .expect("y coordinate should be numeric");
            let z = s
                .next()
                .expect("line should have z coordinate")
                .parse()
                .expect("z coordinate should be numeric");
            return (x, y, z);
        })
        .collect();

    let sides: usize = points
        .iter()
        .map(|p| {
            let deltas = [
                (-1, 0, 0),
                (1, 0, 0),
                (0, -1, 0),
                (0, 1, 0),
                (0, 0, -1),
                (0, 0, 1),
            ];
            return deltas
                .iter()
                .filter(|d| !points.contains(&(p.0 + d.0, p.1 + d.1, p.2 + d.2)))
                .count();
        })
        .sum();

    println!("{}", sides);
}
