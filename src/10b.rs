use std::io;

fn output_beam(cycle: i32, pos: i32) {
    let beam_pos = cycle % 40;
    if beam_pos.abs_diff(pos) <= 1 {
        print!("#");
    } else {
        print!(".");
    }
    if beam_pos == 39 {
        println!();
    }
}

fn main() {
    let mut x = 1;
    let mut add_op = None;

    for i in 0..240 {
        match add_op {
            None => {
                let mut line = String::new();
                let _ = io::stdin()
                    .read_line(&mut line)
                    .expect("should have enough input lines for this cycle");
                let mut split = line.split_ascii_whitespace();
                let inst = split.next().expect("line should have instruction");

                if inst == "addx" {
                    let op: i32 = split
                        .next()
                        .expect("line should have operand for addx instruction")
                        .parse()
                        .expect("operand should be numeric");
                    add_op = Some(op);
                }

                output_beam(i, x);
            }
            Some(data) => {
                output_beam(i, x);
                x += data;
                add_op = None;
            }
        };
    }
}
