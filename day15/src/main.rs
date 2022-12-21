use std::collections::HashSet;

use vector2d::Vector2D;

#[derive(Debug)]
struct Sensor {
    pos: Vector2D<isize>,
    closest_beacon: Vector2D<isize>,
    distance: isize,
}

fn parser(s: String) -> Vec<Sensor> {
    //Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    let re = regex::Regex::new(
        r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
    )
    .unwrap();

    let mut sensors = vec![];
    for line in s.split('\n') {
        let caps = re.captures(line).unwrap();
        let (sx, sy): (isize, isize) = (
            caps.get(1).unwrap().as_str().parse().unwrap(),
            caps.get(2).unwrap().as_str().parse().unwrap(),
        );
        let (bx, by): (isize, isize) = (
            caps.get(3).unwrap().as_str().parse().unwrap(),
            caps.get(4).unwrap().as_str().parse().unwrap(),
        );

        let sensor = Sensor {
            pos: Vector2D { x: sx, y: sy },
            closest_beacon: Vector2D { x: bx, y: by },
            distance: manhattan_distance(sx, sy, bx, by),
        };

        sensors.push(sensor);
    }
    sensors
}

fn manhattan_distance(a_x: isize, a_y: isize, b_x: isize, b_y: isize) -> isize {
    (a_x - b_x).abs() + (a_y - b_y).abs()
}

fn solve(sensors: &[Sensor], filter_row: isize) -> usize {
    let mut affected_cols = HashSet::new();
    for sensor in sensors {
        // Filter sensors that don't affect filter_row
        if sensor.pos.y + sensor.distance < filter_row
            || sensor.pos.y - sensor.distance > filter_row
        {
            continue;
        }

        for x in sensor.pos.x - sensor.distance..=sensor.pos.x + sensor.distance {
            if sensors
                .iter()
                .find_map(|c| {
                    if (c.closest_beacon.x == x && c.closest_beacon.y == filter_row)
                        || (c.pos.x == x && c.pos.y == filter_row)
                    {
                        Some(x)
                    } else {
                        None
                    }
                })
                .is_some()
            {
                // Don't overwrite signals or beacons
                continue;
            }
            if manhattan_distance(sensor.pos.x, sensor.pos.y, x, filter_row) <= sensor.distance {
                affected_cols.insert(x);
            }
        }
    }
    affected_cols.len()
}

fn solve2(sensors: &[Sensor], max_val: isize) -> isize {
    // x * 4_000_000 + y
    let mut x = 0;
    let mut y = 0;

    // Explore the boundaries of each sensor
    'outer: for sensor in sensors {
        x = sensor.pos.x - sensor.distance;
        y = sensor.pos.y - 1;
        while x < sensor.pos.x {
            // Left-Upper side
            x += 1;
            y -= 1;
            if x < 0 || x > max_val || y < 0 || y > max_val {
                continue;
            }
            if !sensors.iter().any(|sensor| {
                manhattan_distance(sensor.pos.x, sensor.pos.y, x, y) <= sensor.distance
            }) {
                break 'outer;
            }
        }
        while y < sensor.pos.y {
            // Right-Upper side
            x += 1;
            y += 1;
            if x < 0 || x > max_val || y < 0 || y > max_val {
                continue;
            }
            if !sensors.iter().any(|sensor| {
                manhattan_distance(sensor.pos.x, sensor.pos.y, x, y) <= sensor.distance
            }) {
                break 'outer;
            }
        }
        while x > sensor.pos.x {
            // Right-Lower side
            x -= 1;
            y += 1;
            if x < 0 || x > max_val || y < 0 || y > max_val {
                continue;
            }
            if !sensors.iter().any(|sensor| {
                manhattan_distance(sensor.pos.x, sensor.pos.y, x, y) <= sensor.distance
            }) {
                break 'outer;
            }
        }
        while y > sensor.pos.y {
            // Left-Lower side
            x -= 1;
            y -= 1;
            if x < 0 || x > max_val || y < 0 || y > max_val {
                continue;
            }
            if !sensors.iter().any(|sensor| {
                manhattan_distance(sensor.pos.x, sensor.pos.y, x, y) <= sensor.distance
            }) {
                break 'outer;
            }
        }
    }
    x * 4_000_000 + y
}

fn main() {
    let sensors = utils::load_puzzle_data(15, parser);
    let spaces_not_beacon_row_2000000 = solve(sensors.as_slice(), 2000000);
    println!("Solution 1: There are {spaces_not_beacon_row_2000000} that are not beacons on row 2000000.");

    let tuning_freq = solve2(sensors.as_slice(), 4_000_000);
    println!("Solution 2: The tuning frequency of the beacon is: {tuning_freq}");
}

#[cfg(test)]
mod tests {
    use crate::{parser, solve, solve2};

    #[test]
    fn test_puzzle() {
        let test_data = utils::load_puzzle_test(15, parser);
        let solution = solve(test_data.as_slice(), 10);
        assert_eq!(solution, 26);
    }

    #[test]
    fn test_puzzle2() {
        let test_data = utils::load_puzzle_test(15, parser);
        let solution = solve2(test_data.as_slice(), 20);
        assert_eq!(solution, 56000011);
    }
}
