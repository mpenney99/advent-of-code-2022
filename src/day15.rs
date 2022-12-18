use std::collections::HashSet;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::utils::read_lines;

const ROW_Y: i32 = 2000000;

type Cell = (i32, i32);

#[derive(Debug, PartialEq)]
struct Sensor {
    pos: Cell,
    closest_beacon: Cell,
    dist: i32,
}

fn get_distance(from: Cell, to: Cell) -> i32 {
    (from.0 - to.0).abs() + (from.1 - to.1).abs()
}

fn parse_sensor_reading(line: &str) -> Option<Sensor> {
    static REGEX: Lazy<Regex> = Lazy::<Regex>::new(|| {
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap()
    });

    let captures = REGEX.captures(line)?;
    let x = captures.get(1)?.as_str().parse::<i32>().unwrap();
    let y = captures.get(2)?.as_str().parse::<i32>().unwrap();
    let pos: Cell = (x, y);

    let beacon_x = captures.get(3)?.as_str().parse::<i32>().unwrap();
    let beacon_y = captures.get(4)?.as_str().parse::<i32>().unwrap();
    let closest_beacon: Cell = (beacon_x, beacon_y);

    Some(Sensor {
        pos,
        closest_beacon,
        dist: get_distance(pos, closest_beacon),
    })
}

fn parse_sensor_readings() -> Vec<Sensor> {
    read_lines("./src/day15_input")
        .map(|s| parse_sensor_reading(s.as_str()).expect(s.as_str()))
        .collect()
}

#[allow(dead_code)]
pub fn problem1() -> i32 {
    let mut sensors: Vec<Sensor> = parse_sensor_readings();
    sensors.sort_by_key(|r| {
        let diff_y: i32 = (ROW_Y - r.pos.1).abs();
        r.pos.0 - r.dist + diff_y
    });

    let mut count: i32 = 0;
    let mut max_x = i32::MIN;
    let mut seen_beacons: HashSet<Cell> = HashSet::new();

    for r in sensors {
        let diff_y: i32 = (ROW_Y - r.pos.1).abs();
        let start: i32 = r.pos.0 - r.dist + diff_y;
        let end: i32 = r.pos.0 + r.dist - diff_y;

        let start = start.max(max_x + 1);

        if end >= start {
            count += end - start + 1;
            max_x = end;
            if r.closest_beacon.1 == ROW_Y && !seen_beacons.contains(&r.closest_beacon) {
                count -= 1;
                seen_beacons.insert(r.closest_beacon);
            }
        }
    }

    println!("{}", count);
    count
}

#[allow(dead_code)]
pub fn problem2() {
    let sensors: Vec<Sensor> = parse_sensor_readings();

    fn is_all_out_of_range(sensors: &Vec<Sensor>, pos: Cell) -> bool {
        sensors.iter().all(|r| r.dist < get_distance(r.pos, pos))
    }

    for r in &sensors {
        for mx in [-1, 1] {
            for my in [-1, 1] {
                for i in 0..=r.dist {
                    let cell: Cell = (
                        (r.pos.0 + (r.dist + 1 - i) * mx),
                        (r.pos.1 + (r.dist - (r.dist - i)) * my)
                    );
                    if cell.0 >= 0 && cell.0 <= 4000000 && cell.1 >= 0 && cell.1 <= 4000000 && is_all_out_of_range(&sensors, cell) {
                        println!("{}", (cell.0 as i64) * (4000000 as i64) + (cell.1 as i64));
                        return;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sensor_reading() {
        let res = parse_sensor_reading(
            "Sensor at x=2150774, y=3136587: closest beacon is at x=-2561642, y=2914773",
        );
        assert_eq!(
            res,
            Some(Sensor {
                pos: (2150774, 3136587),
                closest_beacon: (-2561642, 2914773),
                dist: 4934230
            })
        )
    }

    #[test]
    fn test() {
        problem2();
    }
}
