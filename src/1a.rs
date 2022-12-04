use std::io;

fn main() {
    let mut max = 0;
    let mut sum = 0;

    for line in io::stdin().lines() {
        let line = line.unwrap();

        if line.len() == 0 {
            if sum > max {
                max = sum;
            }
            sum = 0;
        } else {
            let cal: u32 = line
                .trim()
                .parse()
                .expect("calorie value should be numeric");
            sum += cal;
        }
    }

    if sum > max {
        max = sum;
    }
    println!("{}", max);
}
