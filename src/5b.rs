use std::io;
use std::vec::Vec;

fn main() {
    let mut stack_lines = Vec::new();

    // Read in lines containing initial stack arrangement.
    loop {
        let mut line = String::new();
        let _ = io::stdin().read_line(&mut line);

        if line
            .chars()
            .nth(1)
            .expect("stack line should have at least 4 characters for at least 1 stack")
            == '1'
        {
            break;
        }

        stack_lines.push(line);
    }

    stack_lines.reverse();

    // Number of stacks can be determined from the length of a line.
    // len = 4n (since each stack requires 3 chars for the crate, plus 1 to
    // separate it from the other stacks, plus a \n at the end of the line)
    let num_stacks = stack_lines[0].len() / 4;

    let mut stacks = vec![String::new(); num_stacks];
    for line in stack_lines {
        for i in 0..num_stacks {
            let maybe_crate = line
                .chars()
                .nth(4 * i + 1)
                .expect("stack line should have crate name or space");
            if maybe_crate != ' ' {
                stacks[i].push(maybe_crate);
            }
        }
    }

    // Read through rearrangement procedure. Skip blank separator line.
    for line in io::stdin().lines().skip(1) {
        let line = line.unwrap();
        let line_str = line.as_str();
        let procedure: Vec<u32> = line_str
            .split_ascii_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let amt = procedure
            .get(0)
            .expect("procedure should be properly formed with 3 parameters");
        let src = procedure
            .get(1)
            .expect("procedure should be properly formed with 3 parameters");
        let dst = procedure
            .get(2)
            .expect("procedure should be properly formed with 3 parameters");

        let src_stack = stacks
            .get_mut(*src as usize - 1)
            .expect("procedure should refer to a valid source stack");
        let cr = src_stack.split_off(src_stack.len() - *amt as usize);
        let dst_stack = stacks
            .get_mut(*dst as usize - 1)
            .expect("procedure should refer to a valid destination stack");
        dst_stack.push_str(cr.as_str());
    }

    // Output tops of stacks.
    println!(
        "{}",
        stacks
            .iter()
            .filter_map(|s| s.chars().last())
            .collect::<String>()
    );
}
