use std::cmp::{min, max};
use std::collections::HashMap;

fn main() {
    let input_path = std::env::args().skip(1).next().expect("no input");
    let contents = std::fs::read_to_string(input_path).expect("cannot read input");
    let scan = parse_scan(&contents);

    {
        let mut cave = Cave::from_scan(&scan);
        println!("this much sand: {}", how_much_is_the_sand(&mut cave));
    }
    {
        let mut cave = Cave::from_scan(&scan);
        println!("this much time: {}", how_long_until_outlet_blocked(&mut cave));
    }
}

type Segment = Vec<(i32, i32)>;

fn parse_scan(s: &str) -> Vec<Segment> {
    s.lines().map(|line|
        line.split(" -> ")
            .map(|t| match t.split_once(",") {
                Some((a, b)) => (a.parse().expect("a"), b.parse().expect("b")),
                None => panic!("not a tuple")
            }).collect()
    ).collect()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Sand,
    Rock,
}

#[derive(Debug)]
struct Cave {
    items: HashMap<(i32, i32), Cell>,
}

impl Cave {
    fn new() -> Cave {
        Cave{ items: HashMap::new() }
    }

    fn from_scan(segments: &Vec<Segment>) -> Cave {
        let mut cave = Cave::new();

        for s in segments {
            let mut coords = s.iter().peekable();
            let mut start = coords.next().unwrap();
            loop {
                match coords.next() {
                    None => break,
                    Some(c) => {
                        cave.fill(start, c, Cell::Rock);
                        start = c
                    }
                }
            }
        }

        cave
    }

    fn fill(&mut self, from: &(i32, i32), to: &(i32, i32), what: Cell) {
        for i in min(from.0, to.0)..=max(from.0, to.0) {
            for k in min(from.1, to.1)..=max(from.1, to.1) {
                self.items.insert((i,k), what);
            }
        }
    }

    fn max_y(&self) -> Option<&i32> {
        self.items.keys().map(|(_, y)| y).max()
    }

    fn drop_sand(&mut self, from: (i32, i32), abyss: i32) -> bool {
        let mut pos = from;
        loop {
            if pos.1 >= abyss {
                return false;
            }

            pos = if self.items.get(&(pos.0, pos.1 + 1)) == None {
                (pos.0, pos.1 + 1)
            } else if self.items.get(&(pos.0 - 1, pos.1 + 1)) == None {
                (pos.0 - 1, pos.1 + 1)
            } else if self.items.get(&(pos.0 + 1, pos.1 + 1)) == None {
                (pos.0 + 1, pos.1 + 1)
            } else {
                // Rock and ... sand
                break
            }
        }

        self.items.insert(pos, Cell::Sand);
        true
    }

    fn drop_sand_on_bedrock(&mut self, from: (i32, i32), bedrock: i32) {
        let mut pos = from;
        loop {
            if pos.1 >= bedrock {
                break
            }

            pos = if self.items.get(&(pos.0, pos.1 + 1)) == None {
                (pos.0, pos.1 + 1)
            } else if self.items.get(&(pos.0 - 1, pos.1 + 1)) == None {
                (pos.0 - 1, pos.1 + 1)
            } else if self.items.get(&(pos.0 + 1, pos.1 + 1)) == None {
                (pos.0 + 1, pos.1 + 1)
            } else {
                // Rock and ... sand
                break
            }
        }

        self.items.insert(pos, Cell::Sand);
    }
}

fn how_much_is_the_sand(g: &mut Cave) -> usize {
    let abyss = *g.max_y().unwrap_or(&0);
    (0..).take_while(|_| g.drop_sand((500, 0), abyss)).count()
}

fn how_long_until_outlet_blocked(g: &mut Cave) -> usize {
    let bedrock = *g.max_y().unwrap_or(&0) + 1;
    (0..).take_while(|_| {
        g.drop_sand_on_bedrock((500, 0), bedrock);
        g.items.get(&(500, 0)) == None
    }).count() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> &'static str {
        "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"
    }

    #[test]
    fn test_parse_scan() {
        assert_eq!(parse_scan(sample()), vec![
            vec![(498, 4), (498, 6), (496, 6)],
            vec![(503, 4), (502, 4), (502, 9), (494, 9)],
        ]);
    }

    #[test]
    fn test_cave_from_scan() {
        let scan = parse_scan(sample());
        let g = Cave::from_scan(&scan);
        assert_eq!(g.items.get(&(494, 9)), Some(&Cell::Rock));
        assert_eq!(g.items.get(&(495, 9)), Some(&Cell::Rock));
        assert_eq!(g.items.get(&(496, 9)), Some(&Cell::Rock));
        assert_eq!(g.items.get(&(496, 6)), Some(&Cell::Rock));
        assert_eq!(g.items.get(&(497, 9)), Some(&Cell::Rock));
        assert_eq!(g.items.get(&(497, 6)), Some(&Cell::Rock));
        assert_eq!(g.items.get(&(498, 9)), Some(&Cell::Rock));
        assert_eq!(g.items.get(&(498, 6)), Some(&Cell::Rock));
        assert_eq!(g.items.get(&(498, 5)), Some(&Cell::Rock));
        assert_eq!(g.items.get(&(498, 4)), Some(&Cell::Rock));
        assert_eq!(g.items.get(&(499, 9)), Some(&Cell::Rock));
        assert_eq!(g.items.get(&(500, 9)), Some(&Cell::Rock));
        assert_eq!(g.items.get(&(501, 9)), Some(&Cell::Rock));
        assert_eq!(g.items.get(&(502, 9)), Some(&Cell::Rock));
        assert_eq!(g.items.get(&(502, 8)), Some(&Cell::Rock));
        assert_eq!(g.items.get(&(502, 7)), Some(&Cell::Rock));
        assert_eq!(g.items.get(&(502, 6)), Some(&Cell::Rock));
        assert_eq!(g.items.get(&(502, 5)), Some(&Cell::Rock));
        assert_eq!(g.items.get(&(502, 4)), Some(&Cell::Rock));
        assert_eq!(g.items.get(&(503, 4)), Some(&Cell::Rock));
    }

    #[test]
    fn test_drop_sand() {
        let scan = parse_scan(sample());
        let mut g = Cave::from_scan(&scan);
        let abyss = *g.max_y().unwrap_or(&0);
        assert_eq!(g.drop_sand((500, 0), abyss), true);
        assert_eq!(g.items.get(&(500, 8)), Some(&Cell::Sand));
    }

    #[test]
    fn test_drop_more_sand() {
        let scan = parse_scan(sample());
        let mut g = Cave::from_scan(&scan);
        let abyss = *g.max_y().unwrap_or(&0);

        assert_eq!(g.drop_sand((500, 0), abyss), true);
        assert_eq!(g.drop_sand((500, 0), abyss), true);
        assert_eq!(g.drop_sand((500, 0), abyss), true);
        assert_eq!(g.drop_sand((500, 0), abyss), true);
        assert_eq!(g.drop_sand((500, 0), abyss), true);

        assert_eq!(g.items.get(&(500, 7)), Some(&Cell::Sand));
        assert_eq!(g.items.get(&(498, 8)), Some(&Cell::Sand));
        assert_eq!(g.items.get(&(499, 8)), Some(&Cell::Sand));
        assert_eq!(g.items.get(&(500, 8)), Some(&Cell::Sand));
        assert_eq!(g.items.get(&(501, 8)), Some(&Cell::Sand));
    }

    #[test]
    fn test_drop_even_more_sand() {
        let scan = parse_scan(sample());
        let mut g = Cave::from_scan(&scan);
        let abyss = *g.max_y().unwrap_or(&0);

        for _ in 0..24 {
            assert_eq!(g.drop_sand((500, 0), abyss), true);
        }

        assert_eq!(g.items.get(&(500, 2)), Some(&Cell::Sand));
        for i in 499..=501 {
            for k in 3..=8 {
                assert_eq!(g.items.get(&(i, k)), Some(&Cell::Sand));
            }
        }
        assert_eq!(g.items.get(&(498, 7)), Some(&Cell::Sand));
        assert_eq!(g.items.get(&(498, 8)), Some(&Cell::Sand));
        assert_eq!(g.items.get(&(497, 8)), Some(&Cell::Sand));

        assert_eq!(g.items.get(&(497, 5)), Some(&Cell::Sand));
        assert_eq!(g.items.get(&(495, 8)), Some(&Cell::Sand));
    }

    #[test]
    fn test_drop_too_much_sand() {
        let scan = parse_scan(sample());
        let mut g = Cave::from_scan(&scan);
        let abyss = *g.max_y().unwrap_or(&0);

        for _ in 0..24 {
            assert_eq!(g.drop_sand((500, 0), abyss), true);
        }
        assert_eq!(g.drop_sand((500, 0), abyss), false);
    }

    #[test]
    fn test_how_much_is_the_sand() {
        let scan = parse_scan(sample());
        let mut g = Cave::from_scan(&scan);
        assert_eq!(how_much_is_the_sand(&mut g), 24);
    }

    #[test]
    fn test_how_long_until_outlet_blocked() {
        let scan = parse_scan(sample());
        let mut g = Cave::from_scan(&scan);
        assert_eq!(how_long_until_outlet_blocked(&mut g), 93);
    }
}
