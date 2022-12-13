use std::{env, fs, cmp};
use std::ops::Index;
use std::collections::{HashSet, HashMap};

type Point = (usize, usize);

struct Map {
    map: Vec<u8>,
    cols: usize,
    rows: usize,
    start: Point,
    goal: Point,
}

impl Map {
    fn to_index(i: usize, k: usize, cols: usize) -> usize {
        k + i * cols
    }

    fn to_coords(i: usize, cols: usize) -> Point {
        (i / cols, i % cols) as Point
    }

    fn from_str(s: &str) -> Map {
        let mut lines = s.lines().peekable();

        let mut map: Vec<u8> = vec![];
        let cols: usize = lines.peek().unwrap().len();
        let mut start: Option<Point> = None;
        let mut goal: Option<Point> = None;

        for line in lines {
            for c in line.chars() {
                map.push(match c {
                    'S' => {
                        start = Some(Self::to_coords(map.len(), cols));
                        0
                    },
                    'E' => {
                        goal = Some(Self::to_coords(map.len(), cols));
                        25
                    },
                    'a'..='z' => c as u8 - 'a' as u8,
                    _ => unreachable!()
                });
            }
        }
        let rows = map.len() / cols;

        Map{
            map: map,
            cols: cols,
            rows: rows,
            start: start.unwrap(),
            goal: goal.unwrap(),
        }
    }

    fn possible_starting_points(&self) -> Vec<Point> {
        let mut res: Vec<Point> = vec![];
        for i in 0..self.rows {
            for k in 0..self.cols {
                if self.map[Self::to_index(i, k, self.cols)] == 0 {
                    res.push((i, k));
                }
            }
        }
        res
    }
}

impl Index<Point> for Map {
    type Output = u8;

    fn index(&self, p: Point) -> &Self::Output {
        assert!(p.0 < self.rows);
        assert!(p.1 < self.cols);
        &self.map[Self::to_index(p.0, p.1, self.cols)]
    }
}

fn add(p: Point, d: &(i32, i32)) -> Point {
    (((p.0 as i32) + d.0) as usize,
     ((p.1 as i32) + d.1) as usize)
}

fn a_star(m: &Map, start: Point) -> Option<Vec<Point>> {
    let mut open_set: HashSet<Point> = HashSet::from([start]);
    let mut came_from: HashMap<Point, Point> = HashMap::from([]);
    let mut g_score: HashMap<Point, u32> = HashMap::from([(start, 0)]);
    let mut f_score: HashMap<Point, u32> = HashMap::from([
        (start, heuristic(&m, &start))
    ]);

    let deltas: Vec<(i32, i32)> = vec![(0, -1), (0,  1), (-1, 0), ( 1, 0)];

    while !open_set.is_empty() {
        let current = open_set.iter()
            .map(|n| (n, f_score[n]))
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap().0.clone();

        if current.0 == m.goal.0 && current.1 == m.goal.1 {
            let mut node = current;
            let mut path: Vec<Point> = vec![node];
            while came_from.contains_key(&node) {
                node = *came_from.get(&node).unwrap();
                path.push(node);
            }
            path.reverse();
            return Some(path);
        }

        open_set.remove(&current);
        let neighbors = deltas.iter()
            .filter(|d| valid_move(m, &current, **d))
            .map(|d| add(current, d));

        for neighbor in neighbors {
            let tentative_score = g_score[&current] + 1;
            if match g_score.get(&neighbor) {
                None => true,
                Some(score) => tentative_score <= *score
            } {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_score);
                f_score.insert(neighbor, tentative_score + heuristic(&m, &neighbor));
                open_set.insert(neighbor);
            }
        }
    }

    None
}

fn heuristic(m: &Map, p: &Point) -> u32 {
    let q = m.goal;
    let dx = if p.0 > q.0 { p.0 - q.0 } else { q.0 - p.0 };
    let dy = if p.1 > q.1 { p.1 - q.1 } else { q.1 - p.1 };
    cmp::max(dx, dy) as u32
}

fn valid_move(m: &Map, position: &Point, delta: (i32, i32)) -> bool {
    let q = (
        (position.0 as i32 + delta.0) as usize,
        (position.1 as i32 + delta.1) as usize
    );
    q.0 < m.rows && q.1 < m.cols && m[*position] + 1 >= m[q]
}

fn main() {
    let input_path = env::args().skip(1).next().expect("no input");
    let contents = fs::read_to_string(input_path).expect("cannot read");

    let map = Map::from_str(&contents);
    let path = a_star(&map, map.start).unwrap();

    println!("Path length: {}", path.len());

    let starting_points = map.possible_starting_points();
    let shortest_path_len = starting_points.iter()
        .map(|p| a_star(&map, *p))
        .filter(|p| p.is_some())
        .map(|p| p.unwrap().len())
        .min().unwrap();
    println!("shortest path length: {:?}", shortest_path_len);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> &'static str {
        "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"
    }

    #[test]
    fn test_coord_mapping() {
        assert_eq!(Map::to_coords(0, 12), (0, 0));
        assert_eq!(Map::to_index(0, 0, 12), 0);

        let idx = Map::to_index(7, 5, 12);
        assert_eq!(Map::to_coords(idx, 12), (7, 5));

        let c = Map::to_coords(23, 12);
        assert_eq!(Map::to_index(c.0, c.1, 12), 23);
    }

    #[test]
    fn test_map_from_sample() {
        let map = Map::from_str(sample());
        assert_eq!(map.map, vec![
        // 0  1  2   3   4   5   6   7
           0, 0, 1, 16, 15, 14, 13, 12, // 0
           0, 1, 2, 17, 24, 23, 23, 11, // 1
           0, 2, 2, 18, 25, 25, 23, 10, // 2
           0, 2, 2, 19, 20, 21, 22,  9, // 3
           0, 1, 3,  4,  5,  6,  7,  8  // 4
        ]);
        assert_eq!(map.cols, 8);
        assert_eq!(map.rows, 5);
        assert_eq!(map.start, (0, 0));
        assert_eq!(map.goal, (2, 5));

        assert_eq!(map[(0,0)], 0);
        assert_eq!(map[(1,1)], 1);
        assert_eq!(map[(2,3)], 18);
    }

    #[test]
    fn test_move() {
        let map = Map::from_str(sample());
        assert_eq!(valid_move(&map, &map.start, (1, 0)), true);
        assert_eq!(valid_move(&map, &map.start, (0, 1)), true);
        assert_eq!(valid_move(&map, &map.start, (-1, 0)), false);
        assert_eq!(valid_move(&map, &(3,2), (0, 1)), false);
        assert_eq!(valid_move(&map, &(3,2), (1, 0)), true);
        assert_eq!(valid_move(&map, &(4,7), (1, 0)), false);
    }

    #[test]
    fn test_search_path() {
        let map = Map::from_str(sample());
        let path = a_star(&map, map.start);
        assert_ne!(path, None);
        assert_eq!(path.unwrap().len(), 32);
    }

    #[test]
    fn test_starting_points() {
        let map = Map::from_str(sample());
        let starting_points = map.possible_starting_points();
        assert_eq!(starting_points, vec![(0, 0), (0, 1), (1,0), (2,0), (3,0), (4,0)]);
    }
}
