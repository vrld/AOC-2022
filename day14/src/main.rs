use std::cmp::{min, max};
use std::ops::{Index, IndexMut};

fn main() {
    println!("Hello, world!");
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

fn get_bounds(segs: &Vec<Segment>) -> ((i32, i32), (i32, i32)) {
    segs.iter().flatten().fold(
        ((i32::MAX, i32::MIN), (i32::MAX, i32::MIN)),
        |((xmin, xmax), (ymin, ymax)), (x, y)| {
            ((min(*x, xmin), max(*x, xmax)),
             (min(*y, ymin), max(*y, ymax)))
        }
    )
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Air,
    Sand,
    Rock,
}

#[derive(Debug)]
struct Grid {
    width: i32,
    height: i32,
    offset_x: i32,
    items: Vec<Cell>,
}

impl Grid {
    fn new(width: i32, height: i32, offset_x: i32) -> Grid {
        Grid{
            width: width,
            height: height,
            offset_x: offset_x,
            items: vec![Cell::Air; (width * height) as usize],
        }
    }

    fn from_scan(segments: &Vec<Segment>) -> Grid {
        let ((x0, x1), (_, y1)) = get_bounds(&segments);
        let mut g = Grid::new(x1 - x0 + 1, y1 + 1, x0);

        for s in segments {
            let mut coords = s.iter().peekable();
            let mut start = coords.next().unwrap();
            loop {
                match coords.next() {
                    None => break,
                    Some(c) => {
                        g.fill(start, c, Cell::Rock);
                        start = c
                    }
                }
            }
        }

        g
    }

    fn to_index(&self, i: i32, k: i32) -> usize {
        assert!(i >= self.offset_x && i < self.width + self.offset_x);
        assert!(k >= 0 && k < self.height);
        (k * self.width + i - self.offset_x) as usize
    }

    fn fill(&mut self, from: &(i32, i32), to: &(i32, i32), what: Cell) {
        for i in min(from.0, to.0)..=max(from.0, to.0) {
            for k in min(from.1, to.1)..=max(from.1, to.1) {
                self[(i,k)] = what;
            }
        }
    }

    fn get(&self, i: i32, k: i32) -> Option<Cell> {
        if i < self.offset_x || i > self.width + self.offset_x || k < 0 || k >= self.height {
            None
        } else {
            Some(self.items[self.to_index(i, k)])
        }
    }

    fn drop_sand(&mut self, from: (i32, i32)) -> bool {
        let mut pos = from;
        loop {
            match self.get(pos.0, pos.1 + 1) {
                Some(Cell::Air) => {
                    pos = (pos.0, pos.1 + 1);
                    continue
                },
                None => return false,
                _ => (), // Rock and Sand
            }

            match self.get(pos.0 - 1, pos.1 + 1) {
                Some(Cell::Air) => {
                    pos = (pos.0 - 1, pos.1 + 1);
                    continue
                },
                None => return false,
                _ => (), // for Karl!
            }

            match self.get(pos.0 + 1, pos.1 + 1) {
                Some(Cell::Air) => pos = (pos.0 + 1, pos.1 + 1),
                None => return false,
                _ => break,
            }
        }
        self[pos] = Cell::Sand;
        true
    }
}

impl Index<(i32, i32)> for Grid {
    type Output = Cell;

    fn index(&self, idx: (i32, i32)) -> &Self::Output {
        &self.items[self.to_index(idx.0, idx.1)]
    }
}

impl IndexMut<(i32, i32)> for Grid {
    fn index_mut(&mut self, idx: (i32, i32)) -> &mut Self::Output {
        let idx = self.to_index(idx.0, idx.1);
        &mut self.items[idx]
    }
}

fn how_much_is_the_sand(g: &mut Grid) -> usize {
    (0..).take_while(|_| g.drop_sand((500, 0))).count()
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
    fn test_get_bounds() {
        let scan = parse_scan(sample());
        assert_eq!(get_bounds(&scan), ((494, 503), (4, 9)));
    }

    #[test]
    fn test_grid_from_scan() {
        let scan = parse_scan(sample());
        let g = Grid::from_scan(&scan);
        assert_eq!(g.width, 10);
        assert_eq!(g.height, 10);
        assert_eq!(g.offset_x, 494);
        assert_eq!(g.items, vec![
           //    494         495         496         497         498         499         500         501         502         503
           Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  // 0
           Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  // 1
           Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  // 2
           Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  // 3
           Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Rock, Cell::Air,  Cell::Air,  Cell::Air,  Cell::Rock, Cell::Rock, // 4
           Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Rock, Cell::Air,  Cell::Air,  Cell::Air,  Cell::Rock, Cell::Air,  // 5
           Cell::Air,  Cell::Air,  Cell::Rock, Cell::Rock, Cell::Rock, Cell::Air,  Cell::Air,  Cell::Air,  Cell::Rock, Cell::Air,  // 6
           Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Rock, Cell::Air,  // 7
           Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Air,  Cell::Rock, Cell::Air,  // 8
           Cell::Rock, Cell::Rock, Cell::Rock, Cell::Rock, Cell::Rock, Cell::Rock, Cell::Rock, Cell::Rock, Cell::Rock, Cell::Air,  // 9
        ]);
    }

    #[test]
    fn test_drop_sand() {
        let scan = parse_scan(sample());
        let mut g = Grid::from_scan(&scan);
        assert_eq!(g.drop_sand((500, 0)), true);
        assert_eq!(g[(500, 8)], Cell::Sand);
    }

    #[test]
    fn test_drop_more_sand() {
        let scan = parse_scan(sample());
        let mut g = Grid::from_scan(&scan);

        assert_eq!(g.drop_sand((500, 0)), true);
        assert_eq!(g.drop_sand((500, 0)), true);
        assert_eq!(g.drop_sand((500, 0)), true);
        assert_eq!(g.drop_sand((500, 0)), true);
        assert_eq!(g.drop_sand((500, 0)), true);

        assert_eq!(g[(500, 7)], Cell::Sand);
        assert_eq!(g[(498, 8)], Cell::Sand);
        assert_eq!(g[(499, 8)], Cell::Sand);
        assert_eq!(g[(500, 8)], Cell::Sand);
        assert_eq!(g[(501, 8)], Cell::Sand);
    }

    #[test]
    fn test_drop_even_more_sand() {
        let scan = parse_scan(sample());
        let mut g = Grid::from_scan(&scan);

        for _ in 0..24 {
            assert_eq!(g.drop_sand((500, 0)), true);
        }

        assert_eq!(g[(500, 2)], Cell::Sand);
        for i in 499..=501 {
            for k in 3..=8 {
                assert_eq!(g[(i, k)], Cell::Sand);
            }
        }
        assert_eq!(g[(498, 7)], Cell::Sand);
        assert_eq!(g[(498, 8)], Cell::Sand);
        assert_eq!(g[(497, 8)], Cell::Sand);

        assert_eq!(g[(497, 5)], Cell::Sand);
        assert_eq!(g[(495, 8)], Cell::Sand);
    }

    #[test]
    fn test_drop_too_much_sand() {
        let scan = parse_scan(sample());
        let mut g = Grid::from_scan(&scan);

        for _ in 0..24 {
            assert_eq!(g.drop_sand((500, 0)), true);
        }
        assert_eq!(g.drop_sand((500, 0)), false);
    }

    #[test]
    fn test_how_much_is_the_sand() {
        let scan = parse_scan(sample());
        let mut g = Grid::from_scan(&scan);
        assert_eq!(how_much_is_the_sand(&mut g), 24);
    }
}
