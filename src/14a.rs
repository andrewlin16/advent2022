use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use std::io;
use std::rc::Rc;

type Map = HashMap<u32, Rc<RefCell<BTreeSet<u32>>>>;

fn parse_point(s: &str) -> (u32, u32) {
    let split = s.split_once(',').expect("point should have comma");
    return (
        split
            .0
            .parse()
            .expect("point x coordinate should be numeric"),
        split
            .1
            .parse()
            .expect("point y coordinate should be numeric"),
    );
}

fn insert_point(map: &mut Map, p: &(u32, u32)) {
    match map.get(&p.0) {
        Some(ys) => {
            ys.borrow_mut().insert(p.1);
        }
        None => {
            map.insert(p.0, Rc::new(RefCell::new(BTreeSet::from([p.1]))));
        }
    };
}

fn filled(map: &Map, p: &(u32, u32)) -> bool {
    match map.get(&p.0) {
        Some(ys) => ys.borrow().contains(&p.1),
        None => false,
    }
}

fn main() {
    // map is stored as Map<x, Tree<y>>. For any (x, y) that is filled, x will
    // be in map and will contain a tree of the y's that are filled at that x
    // coordinate.
    let mut map = HashMap::new();

    for line in io::stdin().lines() {
        let line = line.unwrap();

        let mut points = line.split(" -> ");
        let mut last_point = parse_point(points.next().expect("path should have first point"));
        'path: loop {
            match points.next() {
                None => {
                    break 'path;
                }
                Some(point) => {
                    let point = parse_point(point);
                    while last_point != point {
                        insert_point(&mut map, &last_point);
                        if last_point.0 == point.0 {
                            if last_point.1 < point.1 {
                                last_point.1 += 1;
                            } else {
                                last_point.1 -= 1;
                            }
                        } else if last_point.1 == point.1 {
                            if last_point.0 < point.0 {
                                last_point.0 += 1;
                            } else {
                                last_point.0 -= 1;
                            }
                        } else {
                            panic!("path points should be axis-aligned");
                        }
                    }
                    insert_point(&mut map, &last_point);
                }
            }
        }
    }

    let mut sands = 0;
    'sand: loop {
        let mut sand = (500, 0);

        'sand_sim: loop {
            let below_point = (sand.0, sand.1 + 1);

            if filled(&map, &below_point) {
                // Tile below sand is blocked.
                let downleft_point = (sand.0 - 1, sand.1 + 1);
                if !filled(&map, &downleft_point) {
                    sand = downleft_point;
                    continue 'sand_sim;
                }

                let downright_point = (sand.0 + 1, sand.1 + 1);
                if !filled(&map, &downright_point) {
                    sand = downright_point;
                    continue 'sand_sim;
                }

                // Sand is at rest because it cannot move further.
                // Add sand point to map and spawn another sand unit.
                insert_point(&mut map, &sand);
                sands += 1;
                break 'sand_sim;
            } else {
                // Tile below sand is not blocked, look for landing point.
                let col = map.get(&sand.0);

                match col {
                    None => {
                        // Nothing on this column, so sand falls into the abyss.
                        break 'sand;
                    }
                    Some(col) => {
                        // Check what's underneath on this column.
                        let col = col.borrow();
                        let y = col.range(sand.1..).next();

                        match y {
                            None => {
                                // Sand falls into the abyss.
                                break 'sand;
                            }
                            Some(settle_y) => {
                                // Sand falls onto something, update position.
                                sand.1 = settle_y - 1;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", sands);
}
