use std::cmp::{min, max};
use std::collections::HashSet;

fn main() {
    let input_path = std::env::args().skip(1).next().expect("no input");
    let contents = std::fs::read_to_string(input_path).expect("cannot read input");

    let sensors = parse_sensors(&contents);
    println!("row coverage on row 2000000: {}", row_coverage(&sensors, 2000000));
}

#[derive(Debug, PartialEq)]
struct Sensor {
    position: (i32, i32),
    closest_beacon: (i32, i32),
    radius: i32,
}

impl Sensor {
    fn from_str(line: &str) -> Sensor {
        let coords: Vec<i32> = line
            .split(&['=', ',', ':'][..])
            .skip(1).step_by(2)
            .map(|x| x.parse().expect("cannot parse number"))
            .collect();
        assert_eq!(coords.len(), 4);

        Sensor{
            position: (coords[0], coords[1]),
            closest_beacon: (coords[2], coords[3]),
            radius: (coords[0] - coords[2]).abs() + (coords[1] - coords[3]).abs()
        }
    }

    fn covers(&self, x: i32, y: i32) -> bool {
        (self.position.0 - x).abs() + (self.position.1 - y).abs() <= self.radius
    }
}

fn parse_sensors(s: &str) -> Vec<Sensor> {
    s.split_terminator("\n").map(Sensor::from_str).collect()
}

fn get_world_bounds(s: &Vec<Sensor>) -> ((i32, i32), (i32, i32)) {
    s.iter().fold(
        ((i32::MAX, i32::MIN), (i32::MAX, i32::MIN)),
        |bounds, s| {
            ((min(bounds.0.0, s.position.0 - s.radius),
              max(bounds.0.1, s.position.0 + s.radius)),
             (min(bounds.1.0, s.position.1 - s.radius),
              max(bounds.1.1, s.position.1 + s.radius)))
        })
}

fn row_coverage(sensors: &Vec<Sensor>, y: i32) -> usize {
    let row_beacons_x: HashSet<i32> = sensors.iter()
        .filter(|s| s.closest_beacon.1 == y)
        .map(|s| s.closest_beacon.0)
        .collect();

    let ((x0, x1), _) = get_world_bounds(&sensors);
    (x0..=x1).filter(|x| !row_beacons_x.contains(x) && sensors.iter().any(|s| s.covers(*x, y))).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> &'static str {
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"
    }

    #[test]
    fn test_smug_sensor_parsing() {
        let line = "Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        let coords: Vec<i32> = line
            .split(&['=', ',', ':'][..])
            .skip(1).step_by(2)
            .map(|x| x.parse().expect("cannot parse number"))
            .collect();
        assert_eq!(coords, vec![20, 1, 15, 3]);
    }

    #[test]
    fn test_sensor_from_string() {
        let line = "Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        let s = Sensor::from_str(line);
        assert_eq!(s.position, (20, 1));
        assert_eq!(s.closest_beacon, (15, 3));
        assert_eq!(s.radius, 7);
    }

    #[test]
    fn test_sensor_covers() {
        let s = Sensor{
            position: (42, 23),
            closest_beacon: (62, 15),
            radius: 28
        };
        assert_eq!(s.covers(42, 23), true);
        assert_eq!(s.covers(62, 15), true);
        assert_eq!(s.covers(50, 19), true);

        assert_eq!(s.covers(56, 8), false);
        assert_eq!(s.covers(666, 23), false);
        assert_eq!(s.covers(0, 0), false);
    }

    #[test]
    fn test_parse_sample() {
        let sensors = parse_sensors(sample());
        assert_eq!(sensors, vec![
           Sensor{ position: (2, 18), closest_beacon: (-2, 15), radius: 4 + 3 },
           Sensor{ position: (9, 16), closest_beacon: (10, 16), radius: 1 },
           Sensor{ position: (13, 2), closest_beacon: (15, 3), radius: 2 + 1 },
           Sensor{ position: (12, 14), closest_beacon: (10, 16), radius: 2 + 2 },
           Sensor{ position: (10, 20), closest_beacon: (10, 16), radius: 4 },
           Sensor{ position: (14, 17), closest_beacon: (10, 16), radius: 4 + 1 },
           Sensor{ position: (8, 7), closest_beacon: (2, 10), radius: 6 + 3 },
           Sensor{ position: (2, 0), closest_beacon: (2, 10), radius: 10 },
           Sensor{ position: (0, 11), closest_beacon: (2, 10), radius: 2 + 1 },
           Sensor{ position: (20, 14), closest_beacon: (25, 17), radius: 5 + 3 },
           Sensor{ position: (17, 20), closest_beacon: (21, 22), radius: 4 + 2 },
           Sensor{ position: (16, 7), closest_beacon: (15, 3), radius: 1 + 4 },
           Sensor{ position: (14, 3), closest_beacon: (15, 3), radius: 1 },
           Sensor{ position: (20, 1), closest_beacon: (15, 3), radius: 5 + 2 },
        ]);
    }

    #[test]
    fn test_row_coverage() {
        let sensors = parse_sensors(sample());
        assert_eq!(row_coverage(&sensors, 10), 26);
    }
}
