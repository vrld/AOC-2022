use std::{env,fs};
use std::collections::HashSet;
use std::cmp::{min, max};

type Point = (i32, i32);

#[derive(Debug)]
struct Rope {
    pos_head: Point,
    pos_tail: Point,
    tail_visited: HashSet<Point>,
}

enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Rope {
    fn new() -> Rope {
        Rope {
            pos_head: (0,0),
            pos_tail: (0,0),
            tail_visited: HashSet::new(),
        }
    }

    fn mov(&mut self, d: &Direction) {
        let delta = match d {
            Direction::Up => (0, 1),
            Direction::Right => (1, 0),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
        };

        self.pos_head = (self.pos_head.0 + delta.0, self.pos_head.1 + delta.1);

        let (dx, dy) = (self.pos_head.0 - self.pos_tail.0, self.pos_head.1 - self.pos_tail.1);
        if dx.abs() > 1 || dy.abs() > 1 {
            self.pos_tail = (self.pos_tail.0 + min(1, max(-1, dx)), self.pos_tail.1 + min(1, max(-1, dy)))
        }

        self.tail_visited.insert(self.pos_tail);
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
    let mut rope = Rope::new();
    let input_path = env::args().skip(1).next().expect("give input file");
    for line in fs::read_to_string(input_path).expect("cannot read input").lines() {
        rope.command(line);
    }
    println!("tail visited {} fields", rope.tail_visited.len())
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

    #[test]
    fn test_mov() {
        /*
         * ....  .... .... .... .... .... ....
         * ....  .... H... .H.. .TH. .T.. ....
         * ....  H... T... T... .... ..H. ..TH
         * H...  T... .... .... .... .... ....
         *
         */
        let mut r = Rope::new();
        assert_eq!(r.pos_head, (0, 0));
        assert_eq!(r.pos_tail, (0, 0));

        r.mov(Direction::Up);
        assert_eq!(r.pos_head, (0, 1));
        assert_eq!(r.pos_tail, (0, 0));

        r.mov(Direction::Up);
        assert_eq!(r.pos_head, (0, 2));
        assert_eq!(r.pos_tail, (0, 1));

        r.mov(Direction::Right);
        assert_eq!(r.pos_head, (1, 2));
        assert_eq!(r.pos_tail, (0, 1));

        r.mov(Direction::Right);
        assert_eq!(r.pos_head, (2, 2));
        assert_eq!(r.pos_tail, (1, 2));

        r.mov(Direction::Down);
        assert_eq!(r.pos_head, (2, 1));
        assert_eq!(r.pos_tail, (1, 2));

        r.mov(Direction::Right);
        assert_eq!(r.pos_head, (3, 1));
        assert_eq!(r.pos_tail, (2, 1));

        r.mov(Direction::Left);
        assert_eq!(r.pos_head, (2, 1));
        assert_eq!(r.pos_tail, (2, 1));

        r.mov(Direction::Left);
        assert_eq!(r.pos_head, (1, 1));
        assert_eq!(r.pos_tail, (2, 1));

        r.mov(Direction::Left);
        assert_eq!(r.pos_head, (0, 1));
        assert_eq!(r.pos_tail, (1, 1));

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
        let mut r = Rope::new();
        for line in sample().lines() {
            r.command(line);
        }
        assert_eq!(r.tail_visited.len(), 13);
    }
}
