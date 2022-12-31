use std::io;

fn snafu_to_dec(s: &String) -> i64 {
    let mut acc = 0;
    for c in s.chars() {
        acc = 5 * acc
            + match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => unreachable!("SNAFU number should contain one of =, -, 0, 1, 2"),
            };
    }

    return acc;
}

fn dec_to_snafu(mut v: i64) -> String {
    let mut acc = String::new();

    while v > 0 {
        let rem = v % 5;
        v /= 5;

        acc.push(match rem {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => {
                v += 1;
                '='
            }
            4 => {
                v += 1;
                '-'
            }
            _ => unreachable!("rem must be less than 5"),
        });
    }

    return acc.chars().rev().collect();
}

fn main() {
    let sum = io::stdin().lines().map(|l| snafu_to_dec(&l.unwrap())).sum();
    println!("{}", dec_to_snafu(sum));
}
