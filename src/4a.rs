use std::io;
use std::vec::Vec;

fn main() {
    let mut count = 0;

    for line in io::stdin().lines() {
        let line = line.unwrap();
        let line_str = line.as_str();
        let bounds: Vec<u32> = line_str
            .splitn(4, |c| c == '-' || c == ',')
            .map(|s| s.parse::<u32>().expect("bound value should be numeric"))
            .collect();

        if (bounds[0] <= bounds[2] && bounds[1] >= bounds[3])
            || (bounds[0] >= bounds[2] && bounds[1] <= bounds[3])
        {
            count += 1;
        }
    }

    println!("{}", count);
}
