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

    'group_loop: loop {
        // Read and process line 1 of group by adding its contents to a set.
        let mut line = String::new();
        let num_bytes = io::stdin()
            .read_line(&mut line)
            .expect("failed to read input");
        if num_bytes == 0 {
            break;
        }

        let mut items = HashSet::new();

        for b in line.bytes() {
            items.insert(b);
        }

        // Read and process line 2 of group by paring down set to the common items.
        line.clear();
        io::stdin()
            .read_line(&mut line)
            .expect("failed to read input");
        let old_items = items.clone();
        items.clear();

        for b in line.bytes() {
            if old_items.contains(&b) {
                items.insert(b);
            }
        }

        // Read and process line 3 of group by finding common item.
        line.clear();
        io::stdin()
            .read_line(&mut line)
            .expect("failed to read input");
        for b in line.bytes() {
            if items.contains(&b) {
                sum += item_priority(b);
                continue 'group_loop;
            }
        }
    }

    println!("{}", sum);
}
