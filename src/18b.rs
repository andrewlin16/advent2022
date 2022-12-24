use std::cmp;
use std::collections::HashSet;
use std::io;
use std::vec::Vec;

const DELTAS: [(i8, i8, i8); 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

fn surface_area(points: &HashSet<(i8, i8, i8)>) -> usize {
    return points
        .iter()
        .map(|p| {
            return DELTAS
                .iter()
                .filter(|d| !points.contains(&(p.0 + d.0, p.1 + d.1, p.2 + d.2)))
                .count();
        })
        .sum();
}

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

    let init_point = *points.iter().next().unwrap();
    let bounds = points
        .iter()
        .fold((init_point, init_point), |bound, point| {
            (
                (
                    cmp::min(bound.0 .0, point.0 - 1),
                    cmp::min(bound.0 .1, point.1 - 1),
                    cmp::min(bound.0 .2, point.2 - 1),
                ),
                (
                    cmp::max(bound.1 .0, point.0 + 1),
                    cmp::max(bound.1 .1, point.1 + 1),
                    cmp::max(bound.1 .2, point.2 + 1),
                ),
            )
        });

    let ranges = (
        bounds.0 .0..=bounds.1 .0,
        bounds.0 .1..=bounds.1 .1,
        bounds.0 .2..=bounds.1 .2,
    );

    let mut outer_flood_fill = HashSet::from([bounds.0]);
    let mut search = Vec::from([bounds.0]);

    while !search.is_empty() {
        let p = search.pop().unwrap();

        for d in DELTAS {
            let dp = (p.0 + d.0, p.1 + d.1, p.2 + d.2);

            if !(ranges.0.contains(&dp.0) && ranges.1.contains(&dp.1) && ranges.2.contains(&dp.2)) {
                continue;
            }

            if outer_flood_fill.contains(&dp) {
                continue;
            }

            if points.contains(&dp) {
                continue;
            }

            outer_flood_fill.insert(dp);
            search.push(dp);
        }
    }

    let total_area = surface_area(&outer_flood_fill);

    let x: usize = (bounds.1 .0 - bounds.0 .0 + 1).try_into().unwrap();
    let y: usize = (bounds.1 .1 - bounds.0 .1 + 1).try_into().unwrap();
    let z: usize = (bounds.1 .2 - bounds.0 .2 + 1).try_into().unwrap();
    let outer_area = 2 * (x * y + x * z + y * z);

    let inner_area = total_area - outer_area;
    println!("{}", inner_area);
}
