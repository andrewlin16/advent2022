use std::collections::{HashMap, HashSet};
use std::io;

struct Motion {
    dir: char,
    steps: u32,
}

impl Motion {
    fn from(s: String) -> Motion {
        let mut s = s.split_ascii_whitespace();
        let dir = s
            .next()
            .expect("motion should have direction")
            .chars()
            .nth(0)
            .expect("motion direction should be non-empty");
        let steps = s
            .next()
            .expect("motion should have steps")
            .parse()
            .expect("motion steps should be numeric");

        return Motion {
            dir: dir,
            steps: steps,
        };
    }
}

fn main() {
    let directions: HashMap<char, (i32, i32)> =
        HashMap::from([('L', (-1, 0)), ('R', (1, 0)), ('U', (0, 1)), ('D', (0, -1))]);

    let mut positions = HashSet::new();
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);

    for m in io::stdin().lines().map(|s| Motion::from(s.unwrap())) {
        for _ in 0..m.steps {
            let delta = directions
                .get(&m.dir)
                .expect("motion direction should be L, R, U, or D");
            head.0 += delta.0;
            head.1 += delta.1;

            if head.0 == tail.0 {
                let diff = head.1.abs_diff(tail.1);
                if diff > 1 {
                    tail.1 += (head.1 - tail.1).signum();
                }
            } else if head.1 == tail.1 {
                let diff = head.0.abs_diff(tail.0);
                if diff > 1 {
                    tail.0 += (head.0 - tail.0).signum();
                }
            } else {
                let diff0 = head.0.abs_diff(tail.0);
                let diff1 = head.1.abs_diff(tail.1);

                if diff0 + diff1 > 2 {
                    tail.0 += (head.0 - tail.0).signum();
                    tail.1 += (head.1 - tail.1).signum();
                }
            }

            positions.insert(tail);
        }
    }

    println!("{}", positions.len());
}
