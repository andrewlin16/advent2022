use std::collections::HashSet;
use std::io;

fn item_priority(item: u8) -> u32 {
    if item >= b'a' && item <= b'z' {
        (item - b'a' + 1).into()
    } else if item >= b'A' && item <= b'Z' {
        (item - b'A' + 27).into()
    } else {
        panic!("invalid item")
    }
}

fn main() {
    let mut sum = 0;

    'line_loop: for line in io::stdin().lines() {
        let mut left = line.unwrap();
        let right = left.split_off(left.len() / 2);

        let mut items = HashSet::new();

        for b in left.bytes() {
            items.insert(b);
        }
        for b in right.bytes() {
            if items.contains(&b) {
                sum += item_priority(b);
                continue 'line_loop;
            }
        }
    }

    println!("{}", sum);
}
