use std::collections::HashSet;
use std::io;
use std::vec::Vec;

type Point = (i32, i32);

struct Sensor {
    location: Point,
    radius: u32,
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
    let target: i32 = input
        .next()
        .unwrap()
        .unwrap()
        .parse()
        .expect("target should be numeric");

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

    // Record locations at target coordinate that are within radius of each
    // sensor.
    let mut locations = HashSet::new();

    for sensor in sensors {
        let axis_dist = sensor.location.1.abs_diff(target);
        if axis_dist > sensor.radius {
            continue;
        }

        let remaining_dist = (sensor.radius - axis_dist) as i32;
        let begin = sensor.location.0 - remaining_dist;
        let end = sensor.location.0 + remaining_dist;

        for i in begin..end + 1 {
            locations.insert(i);
        }
    }

    // Remove locations that are actually occupied by beacons.
    for beacon in beacons {
        if beacon.1 == target {
            locations.remove(&beacon.0);
        }
    }

    println!("{}", locations.len());
}
