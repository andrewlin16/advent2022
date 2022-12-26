use std::io;
use std::vec::Vec;

struct Item {
    value: isize,
    moved: bool,
}

fn main() {
    let mut ring: Vec<Item> = io::stdin()
        .lines()
        .map(|l| Item {
            value: l.unwrap().parse().expect("line should be numeric"),
            moved: false,
        })
        .collect();

    let len = ring.len() as isize;
    let mut index = 0;

    for _ in 0..len {
        while ring[index].moved {
            index += 1;
        }

        let mut el = ring.remove(index);
        el.moved = true;

        let new_index = (index as isize + el.value).rem_euclid(len - 1);
        ring.insert(new_index as usize, el);
    }

    let index = ring
        .iter()
        .position(|v| v.value == 0)
        .expect("ring should contain 0");

    let result: isize = [1000, 2000, 3000]
        .iter()
        .map(|i| ring[(index + i) % len as usize].value)
        .sum();
    println!("{}", result);
}
