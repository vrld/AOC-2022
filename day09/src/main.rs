use std::{env,fs};
use std::collections::HashSet;
use std::cmp::{min, max};

type Point = (i32, i32);

#[derive(Debug)]
struct Rope {
    knots: Vec<Point>,
    tail_visited: HashSet<Point>,
}

enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Rope {
    fn new(capacity: usize) -> Rope {
        assert!(capacity >= 2, "must have at least 2 knots");
        Rope {
            knots: vec![(0,0); capacity],
            tail_visited: HashSet::new(),
        }
    }

    fn mov(&mut self, d: &Direction) {
        let head = self.head();
        self.knots[0] = match d {
            Direction::Up => (head.0, head.1 + 1),
            Direction::Right => (head.0 + 1, head.1),
            Direction::Down => (head.0, head.1 - 1),
            Direction::Left => (head.0 - 1, head.1),
        };

        for lead in 0..self.knots.len()-1 {
            self._follow_knot(lead);
        }

        self.tail_visited.insert(self.tail());
    }

    fn head(&self) -> Point {
        *self.knots.first().expect("must have at least 2 knots")
    }

    fn tail(&self) -> Point {
        *self.knots.last().expect("must have at least 2 knots")
    }

    fn _follow_knot(&mut self, lead: usize) {
        let follow = lead + 1;
        assert!(follow < self.knots.len());

        let knot_lead = self.knots[lead];
        let knot_follow = self.knots[follow];

        let (dx, dy) = (knot_lead.0 - knot_follow.0, knot_lead.1 - knot_follow.1);
        if dx.abs() > 1 || dy.abs() > 1 {
            self.knots[follow] = (knot_follow.0 + min(1, max(-1, dx)), knot_follow.1 + min(1, max(-1, dy)))
        }
    }

    fn command(&mut self, cmd: &str) {
        let tokens: Vec<&str> = cmd.split(" ").collect();
        assert_eq!(tokens.len(), 2);
        let direction = match tokens[0] {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => panic!("invalid direction"),
        };
        let count: i32 = tokens[1].parse().expect("not a number");

        for _ in 0..count {
            self.mov(&direction);
        }
    }
}

fn main() {
    let input_path = env::args().skip(1).next().expect("give input file");
    let content = fs::read_to_string(input_path).expect("cannot read input");

    let mut short_rope = Rope::new(2);
    let mut long_rope = Rope::new(10);
    for line in content.lines() {
        short_rope.command(line);
        long_rope.command(line);
    }
    println!("short rope: tail visited {} fields", short_rope.tail_visited.len());
    println!("long rope: tail visited {} fields", long_rope.tail_visited.len());
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
    }

    fn sample2() -> &'static str {
        "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
    }

    #[test]
    fn test_mov() {
        /*
         * ....  .... .... .... .... .... ....
         * ....  .... H... .H.. .TH. .T.. ....
         * ....  H... T... T... .... ..H. ..TH
         * H...  T... .... .... .... .... ....
         *
         */
        let mut r = Rope::new(2);
        assert_eq!(r.head(), (0, 0));
        assert_eq!(r.tail(), (0, 0));

        r.mov(&Direction::Up);
        assert_eq!(r.head(), (0, 1));
        assert_eq!(r.tail(), (0, 0));

        r.mov(&Direction::Up);
        assert_eq!(r.head(), (0, 2));
        assert_eq!(r.tail(), (0, 1));

        r.mov(&Direction::Right);
        assert_eq!(r.head(), (1, 2));
        assert_eq!(r.tail(), (0, 1));

        r.mov(&Direction::Right);
        assert_eq!(r.head(), (2, 2));
        assert_eq!(r.tail(), (1, 2));

        r.mov(&Direction::Down);
        assert_eq!(r.head(), (2, 1));
        assert_eq!(r.tail(), (1, 2));

        r.mov(&Direction::Right);
        assert_eq!(r.head(), (3, 1));
        assert_eq!(r.tail(), (2, 1));

        r.mov(&Direction::Left);
        assert_eq!(r.head(), (2, 1));
        assert_eq!(r.tail(), (2, 1));

        r.mov(&Direction::Left);
        assert_eq!(r.head(), (1, 1));
        assert_eq!(r.tail(), (2, 1));

        r.mov(&Direction::Left);
        assert_eq!(r.head(), (0, 1));
        assert_eq!(r.tail(), (1, 1));

        assert_eq!(r.tail_visited, HashSet::from([
            (0, 0),
            (0, 1),
            (1, 2),
            (2, 1),
            (1, 1),
        ]));
    }

    #[test]
    fn test_sample() {
        let mut r = Rope::new(2);
        for line in sample().lines() {
            r.command(line);
        }
        assert_eq!(r.tail_visited.len(), 13);
    }

    #[test]
    fn test_sample_part2() {
        let mut r = Rope::new(10);
        for line in sample().lines() {
            r.command(line);
        }
        assert_eq!(r.tail_visited.len(), 1);
    }

    #[test]
    fn test_sample2_part2() {
        let mut r = Rope::new(10);
        for line in sample2().lines() {
            r.command(line);
        }
        assert_eq!(r.tail_visited.len(), 36);
    }
}
