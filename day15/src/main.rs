use std::cmp::{min, max};
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

fn main() {
    let input_path = std::env::args().skip(1).next().expect("no input");
    let contents = std::fs::read_to_string(input_path).expect("cannot read input");

    let sensors = parse_sensors(&contents);
    println!("row coverage on row 2000000: {}", row_coverage(&sensors, 2000000));

    let missing_beacons = possible_beacon_positions(&sensors, (0, 4000000), (0, 4000000));
    println!("Possible beacon positions: {:?}", missing_beacons);
    for b in missing_beacons {
        println!("{:?} has tuning frequency {}", b, tuning_frequency(&b))
    }
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
}

fn parse_sensors(s: &str) -> Vec<Sensor> {
    s.split_terminator("\n").map(Sensor::from_str).collect()
}

fn row_coverage(sensors: &Vec<Sensor>, y: i32) -> usize {
    let row_beacon_count  = sensors.iter()
        .filter(|s| s.closest_beacon.1 == y)
        .map(|s| s.closest_beacon)
        .collect::<HashSet<(i32, i32)>>().len();

    world_coverage(&sensors, y, y).get(&y)
        .unwrap().iter()
        .map(|r| r.end() - r.start() + 1).sum::<i32>() as usize - row_beacon_count
}

fn overlaps(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    a.end() >= b.start() && a.start() <= b.end()
}

fn merge(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> Result<RangeInclusive<i32>, &'static str> {
    if overlaps(a, b) || a.end() + 1 == *b.start() {
        Ok(min(*a.start(), *b.start())..=max(*a.end(), *b.end()))
    } else {
        Err("ranges do not overlap")
    }
}

type RangeSet = HashSet<RangeInclusive<i32>>;
type World = HashMap<i32, RangeSet>;

fn insert_row(set: &mut RangeSet, r: &RangeInclusive<i32>) {
    for q in set.clone().iter() {
        match merge(q, &r) {
            Ok(new_range) => {
                set.remove(q);
                return insert_row(set, &new_range);
            },
            _ => (),
        }
    }
    set.insert(r.clone());
}

fn fill_rows(rows: &mut World, s: &Sensor, ymin: i32, ymax: i32) {
    for i in -s.radius..=s.radius {
        let y = s.position.1 + i;
        if y < ymin || y > ymax {
            continue;
        }

        if !rows.contains_key(&y) {
            rows.insert(y, HashSet::new());
        }

        let mut ranges = rows.get_mut(&y).unwrap();
        let r = s.radius - i.abs();
        insert_row(&mut ranges, &((s.position.0 - r)..=(s.position.0 + r)));
    }
}

fn world_coverage(sensors: &Vec<Sensor>, ymin: i32, ymax: i32) -> World {
    let mut world: World = HashMap::new();
    for s in sensors.iter() {
        fill_rows(&mut world, s, ymin, ymax);
    }
    world
}

fn possible_beacon_positions(sensors: &Vec<Sensor>, (x0, x1): (i32, i32), (y0, y1): (i32, i32)) -> Vec<(i32, i32)> {
    let world = world_coverage(&sensors, y0, y1);
    (y0..=y1).map(|y| match world.get(&y) {
        Some(range_set) => {
            let mut ranges: Vec<&RangeInclusive<i32>> = range_set.iter().collect();
            ranges.sort_by(|a, b| a.start().cmp(b.start()));
            (1..ranges.len())
                .map(|i| (max(x0, ranges[i-1].end()+1)..min(x1, *ranges[i].start())).map(|x| (x, y)))
                .flatten().collect::<Vec<(i32, i32)>>()
        },
        None => vec![],
    }).flatten().collect()
}

fn tuning_frequency((x, y): &(i32, i32)) -> usize {
    (*x as usize) * 4000000 + (*y as usize)
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

    #[test]
    fn test_range_overlaps() {
        assert_eq!(overlaps(&(0..=10), &(2..=12)), true);
        assert_eq!(overlaps(&(2..=10), &(0..=4)), true);
        assert_eq!(overlaps(&(2..=4), &(0..=8)), true);
        assert_eq!(overlaps(&(2..=4), &(1..=3)), true);
        assert_eq!(overlaps(&(2..=4), &(1..=2)), true);
        assert_eq!(overlaps(&(2..=4), &(4..=8)), true);
        assert_eq!(overlaps(&(2..=4), &(2..=4)), true);

        assert_eq!(overlaps(&(2..=4), &(5..=4)), false);
        assert_eq!(overlaps(&(9..=10), &(5..=4)), false);
    }

    #[test]
    fn test_fill_rows() {
        let mut w: World = World::new();

        /*   .
         *  ...
         * ....B
         *...S...
         * .....
         *  ...
         *   .
         */
        fill_rows(&mut w, &Sensor{
            position: (0,0),
            closest_beacon: (2,1),
            radius: 3
        }, -5, 5);
        assert_eq!(w, HashMap::from([
            (-3, HashSet::from([0..=0])),
            (-2, HashSet::from([-1..=1])),
            (-1, HashSet::from([-2..=2])),
            ( 0, HashSet::from([-3..=3])),
            ( 1, HashSet::from([-2..=2])),
            ( 2, HashSet::from([-1..=1])),
            ( 3, HashSet::from([0..=0])),
        ]));

        /*   .
         *  ...  .
         * ....B...
         *...S...S..
         * .....B..
         *  ...  .
         *   .
         */
        fill_rows(&mut w, &Sensor{
            position: (4,0),
            closest_beacon: (3,-1),
            radius: 2
        }, -5, 5);
        assert_eq!(w, HashMap::from([
            (-3, HashSet::from([0..=0])),
            (-2, HashSet::from([-1..=1,  4..=4])),
            (-1, HashSet::from([-2..=5])),
            ( 0, HashSet::from([-3..=6])),
            ( 1, HashSet::from([-2..=5])),
            ( 2, HashSet::from([-1..=1, 4..=4])),
            ( 3, HashSet::from([0..=0])),
        ]));
    }

    #[test]
    fn test_possible_beacon_positions() {
        let sensors = parse_sensors(sample());
        let missing_beacons = possible_beacon_positions(&sensors, (0, 20), (0, 20));
        assert_eq!(missing_beacons, vec![(14, 11)]);
        assert_eq!(missing_beacons.iter().map(tuning_frequency).collect::<Vec<i32>>(), vec![56000011]);
    }
}
