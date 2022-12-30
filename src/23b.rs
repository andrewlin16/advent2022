use std::collections::{HashMap, HashSet};
use std::io;

fn get_elf_dest(
    elves: &HashSet<(isize, isize)>,
    elf: (isize, isize),
    round: u16,
) -> Option<(isize, isize)> {
    let elf_n = (elf.0 - 1, elf.1);
    let elf_s = (elf.0 + 1, elf.1);
    let elf_w = (elf.0, elf.1 - 1);
    let elf_e = (elf.0, elf.1 + 1);

    let has_n = elves.contains(&elf_n);
    let has_s = elves.contains(&elf_s);
    let has_w = elves.contains(&elf_w);
    let has_e = elves.contains(&elf_e);
    let has_nw = elves.contains(&(elf.0 - 1, elf.1 - 1));
    let has_ne = elves.contains(&(elf.0 - 1, elf.1 + 1));
    let has_sw = elves.contains(&(elf.0 + 1, elf.1 - 1));
    let has_se = elves.contains(&(elf.0 + 1, elf.1 + 1));

    if !(has_n || has_s || has_w || has_e || has_nw || has_ne || has_sw || has_se) {
        return None;
    } else {
        for i in (0..4_u16).map(|v| (v + round) % 4) {
            match i {
                0 => {
                    if !(has_n || has_ne || has_nw) {
                        return Some(elf_n);
                    }
                }
                1 => {
                    if !(has_s || has_se || has_sw) {
                        return Some(elf_s);
                    }
                }
                2 => {
                    if !(has_w || has_nw || has_sw) {
                        return Some(elf_w);
                    }
                }
                3 => {
                    if !(has_e || has_ne || has_se) {
                        return Some(elf_e);
                    }
                }
                _ => {
                    unreachable!();
                }
            }
        }
    }
    return None;
}

fn move_elves(elves: &mut HashSet<(isize, isize)>, round: u16) -> bool {
    let mut move_map = HashMap::new();
    let mut dest_map = HashMap::new();

    for &elf in elves.iter() {
        if let Some(dest) = get_elf_dest(&elves, elf, round) {
            move_map.insert(elf, dest);
            dest_map.insert(
                dest,
                match dest_map.get(&dest) {
                    Some(v) => v + 1,
                    None => 1,
                },
            );
        }
    }

    if move_map.is_empty() {
        return false;
    }

    for (elf, dest) in move_map {
        if *dest_map.get(&dest).unwrap() == 1 {
            elves.remove(&elf);
            elves.insert(dest);
        }
    }

    return true;
}

fn main() {
    let mut elves = HashSet::new();

    for (i, line) in io::stdin().lines().map(|l| l.unwrap()).enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    elves.insert((i as isize, j as isize));
                }
                '.' => {}
                _ => {
                    panic!("unexpected character in map");
                }
            }
        }
    }

    for i in 0.. {
        let moved = move_elves(&mut elves, i);
        if !moved {
            println!("{}", i + 1);
            break;
        }
    }
}
