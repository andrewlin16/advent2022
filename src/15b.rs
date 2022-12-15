use std::collections::HashSet;
use std::io;
use std::vec::Vec;

type Point = (i32, i32);

struct Sensor {
    location: Point,
    radius: u32,
}

fn distance(p0: &Point, p1: &Point) -> u32 {
    return p0.0.abs_diff(p1.0) + p0.1.abs_diff(p1.1);
}

fn parse_point(s: &str) -> Point {
    let (x_str, y_str) = s
        .split_once(',')
        .expect("point should be of form 'x=_, y=_'");
    return (
        x_str[2..]
            .parse()
            .expect("point x coordinate should be numeric"),
        y_str[3..]
            .parse()
            .expect("point y coordinate should be numeric"),
    );
}

fn main() {
    let mut input = io::stdin().lines();
    let coord_max: i32 = input
        .next()
        .unwrap()
        .unwrap()
        .parse()
        .expect("max coordinate should be numeric");

    // Read in input and store locations + radii of sensors, plus beacons.
    let mut sensors = Vec::new();
    let mut beacons = HashSet::new();

    for line in input {
        let line = line.unwrap();
        let (sensor_str, beacon_str) = line
            .split_once(':')
            .expect("line should have sensor and beacon");

        let sensor = parse_point(&sensor_str[10..]);
        let beacon = parse_point(&beacon_str[22..]);

        sensors.push(Sensor {
            location: sensor,
            radius: sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1),
        });

        beacons.insert(beacon);
    }

    // Find the point which is not covered by the sensors.
    let mut test_loc = (0, 0);

    'check: loop {
        for sensor in &sensors {
            let dist = distance(&sensor.location, &test_loc);
            if dist <= sensor.radius {
                test_loc.0 += (sensor.radius - dist + 1) as i32;
                if test_loc.0 > coord_max {
                    test_loc.0 = 0;
                    test_loc.1 += 1;
                }
                continue 'check;
            }
        }

        // Exhausted all sensors, must have found the uncovered point.
        eprintln!("{:?}", test_loc);
        println!("{}", 4000000 * test_loc.0 as u64 + test_loc.1 as u64);
        return;
    }
}
