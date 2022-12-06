use std::collections::HashMap;
use std::io;

fn inc(map: &mut HashMap<u8, u8>, val: u8) {
    match map.get(&val) {
        Some(v) => {
            map.insert(val, v + 1);
        }
        None => {
            map.insert(val, 1);
        }
    }
}

fn dec(map: &mut HashMap<u8, u8>, val: u8) {
    match map.get(&val) {
        Some(1) => {
            map.remove(&val);
        }
        Some(v) => {
            map.insert(val, v - 1);
        }
        None => {
            panic!("val that is being decremented should be in map");
        }
    }
}

fn main() {
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);

    let mut map = HashMap::new();
    for b in line.bytes().take(14) {
        inc(&mut map, b);
    }

    if map.len() == 14 {
        println!("14");
        return;
    }

    let bytes = line.as_bytes();
    for i in 14..bytes.len() - 1 {
        inc(&mut map, bytes[i]);
        dec(&mut map, bytes[i - 14]);

        if map.len() == 14 {
            println!("{}", i + 1);
            return;
        }
    }
}
