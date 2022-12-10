use std::io;

fn main() {
    let mut x = 1;
    let mut cycle = 1;
    let mut checkpoint = 20;
    let mut signal = 0;

    for line in io::stdin().lines() {
        if cycle > 220 {
            break;
        }

        let line = line.unwrap();
        let mut split = line.split_ascii_whitespace();
        let inst = split.next().expect("line should have instruction");

        if cycle >= checkpoint {
            signal += checkpoint * x;
            checkpoint += 40;
        }

        if inst == "noop" {
            cycle += 1;
        } else if inst == "addx" {
            cycle += 1;

            if cycle >= checkpoint {
                signal += checkpoint * x;
                checkpoint += 40;
            }

            cycle += 1;
            let op: i32 = split
                .next()
                .expect("line should have operand for addx instruction")
                .parse()
                .expect("operand should be numeric");
            x += op;
        } else {
            panic!("expected 'noop' or 'addx' for instruction");
        }
    }

    println!("{}", signal);
}
