use std::io;
use std::vec::Vec;

const KEY: isize = 811589153;

struct Item {
    value: isize,
    index: usize,
}

fn main() {
    let mut ring: Vec<Item> = io::stdin()
        .lines()
        .enumerate()
        .map(|(i, l)| Item {
            value: l.unwrap().parse::<isize>().expect("line should be numeric") * KEY,
            index: i,
        })
        .collect();

    let len = ring.len() as isize;
    let mut index = 0;

    for i in 0..len * 10 {
        while ring[index].index != (i % len) as usize {
            index = (index + 1) % len as usize;
        }

        let el = ring.remove(index);

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
