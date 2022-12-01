use std::collections::BinaryHeap;
use std::io;

fn main() {
    let mut heap = BinaryHeap::new();
    let mut sum = 0;

    for line in io::stdin().lines() {
        let line = line.unwrap();

        if line.len() == 0 {
            heap.push(sum);
            sum = 0;
        } else {
            let cal: u32 = line.trim().parse().expect("invalid value");
            sum += cal;
        }
    }

    heap.push(sum);

    sum = 0;
    for _ in 0..3 {
        sum += heap.pop().expect("insufficient elves");
    }

    println!("{}", sum);
}
