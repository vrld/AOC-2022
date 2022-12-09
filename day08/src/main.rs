use std::ops::Index;
use std::{env, fs};

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    items: Vec<u8>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        Grid{
            width: width,
            height: height,
            items: vec![0; width * height],
        }
    }

    fn from_str(string: &str) -> Grid {
        let mut height: usize = 0;
        let mut width: usize = 0;
        let mut items: Vec<u8> = vec![];
        for line in string.lines() {
            if width == 0 {
                width = line.len();
            }
            assert_eq!(line.len(), width);
            for c in line.chars() {
                items.push(c.to_digit(10).expect("cannot parse digit") as u8);
            }
            height += 1;
        }

        Grid{
            width: width,
            height: height,
            items: items
        }
    }

    fn to_index(&self, i: usize, k: usize) -> usize {
        assert!(i < self.width);
        assert!(k < self.height);
        k * self.width + i
    }

    fn to_coords(&self, i: usize) -> (usize, usize) {
        assert!(i < self.width * self.height);
        (i % self.width, i / self.width)
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = u8;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        &self.items[self.to_index(idx.0, idx.1)]
    }
}

fn dominates_row(grid: &Grid, i: usize, k: usize) -> bool {
    let v = grid[(i, k)];
    (0..i).all(|j| grid[(j, k)] < v)
        || (i+1..grid.width).all(|j| grid[(j, k)] < v)
}

fn dominates_col(grid: &Grid, i: usize, k: usize) -> bool {
    let v = grid[(i, k)];
    (0..k).all(|j| grid[(i, j)] < v)
        || (k+1..grid.height).all(|j| grid[(i, j)] < v)
}

fn is_visible(grid: &Grid, i: usize, k: usize) -> bool {
    i == 0
        || i == (grid.width - 1)
        || k == 0
        || k == (grid.height - 1)
        || dominates_row(grid, i, k)
        || dominates_col(grid, i, k)
}

fn count_visible(grid: &Grid) -> usize {
    (0..grid.width * grid.height)
        .filter(|x| {
            let (i, k) = grid.to_coords(*x);
            is_visible(grid, i, k)
        }).count()
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

fn viewing_distance(grid: &Grid, d: Direction, i: usize, k: usize) -> usize {
    if match d {
        Direction::Up => k == 0,
        Direction::Down => k == grid.height - 1,
        Direction::Left => i == 0,
        Direction::Right => i == grid.width - 1,
    } {
        return 0;
    }

    let v = grid[(i, k)];

    let range: Vec<usize> = match d {
        Direction::Up => (0..k).rev().collect(),
        Direction::Right => (i+1..grid.width).collect(),
        Direction::Down => (k+1..grid.height).collect(),
        Direction::Left => (0..i).rev().collect()
    };

    fn access_row(grid: &Grid, _i: usize, k: usize, j: usize) -> u8 {
        grid[(j, k)]
    }

    fn access_col(grid: &Grid, i: usize, _k: usize, j: usize) -> u8 {
        grid[(i, j)]
    }

    let accessor = match d {
        Direction::Up | Direction::Down => access_col,
        Direction::Left | Direction::Right => access_row,
    };

    let mut distance = 0;
    for j in range.iter() {
        distance += 1;
        if accessor(grid, i, k, *j) >= v {
            break;
        }
    }
    distance
}

fn scening_score(grid: &Grid, i: usize, k: usize) -> usize {
    viewing_distance(grid, Direction::Up, i, k)
        * viewing_distance(grid, Direction::Right, i, k)
        * viewing_distance(grid, Direction::Down, i, k)
        * viewing_distance(grid, Direction::Left, i, k)
}

fn largest_scening_score(grid: &Grid) -> usize {
    (0..grid.width*grid.height).map(|x| {
        let (i, k) = grid.to_coords(x);
        scening_score(grid, i, k)
    }).max().unwrap_or(0)
}

fn main() {
    let input_path = env::args().skip(1).next().expect("give input file");
    let contents = fs::read_to_string(input_path).expect("cannot read input");

    let g = Grid::from_str(&contents);
    println!("# visible trees: {}", count_visible(&g));
    println!("highest scening score: {}", largest_scening_score(&g));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> &'static str {
        "30373
25512
65332
33549
35390"
    }

    #[test]
    fn test_grid_new() {
        let g = Grid::new(3, 2);
        assert_eq!(g.width, 3);
        assert_eq!(g.height, 2);
        assert_eq!(g.items, vec![0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_grid_coords() {
        let g = Grid::new(3, 2);
        assert_eq!(g.to_index(0, 0), 0);
        assert_eq!(g.to_index(2, 0), 2);
        assert_eq!(g.to_index(0, 1), 3);
        assert_eq!(g.to_index(2, 1), 5);

        assert_eq!(g.to_coords(0), (0, 0));
        assert_eq!(g.to_coords(2), (2, 0));
        assert_eq!(g.to_coords(3), (0, 1));
        assert_eq!(g.to_coords(5), (2, 1));
    }

    #[test]
    fn test_grid_from_str() {
        let g = Grid::from_str(sample());
        assert_eq!(g.width, 5);
        assert_eq!(g.height, 5);
        assert_eq!(g.items, vec![
                   3, 0, 3, 7, 3,
                   2, 5, 5, 1, 2,
                   6, 5, 3, 3, 2,
                   3, 3, 5, 4, 9,
                   3, 5, 3, 9, 0
        ]);
    }

    #[test]
    fn test_grid_index() {
        let g = Grid::new(3, 2);
        assert_eq!(g[(0,0)], 0);

        let g2 = Grid::from_str(sample());
        assert_eq!(g2[(0,0)], 3);
        assert_eq!(g2[(2,1)], 5);
        assert_eq!(g2[(2,2)], 3);
        assert_eq!(g2[(4,4)], 0);
    }

    #[test]
    fn test_dominates_row() {
        let g = Grid::from_str(sample());
        assert!(dominates_row(&g, 1, 1));
        assert!(dominates_row(&g, 2, 1));
        assert!(dominates_row(&g, 1, 2));
        assert!(!dominates_row(&g, 2, 2));
    }

    #[test]
    fn test_is_visible() {
        let g = Grid::from_str(sample());
        assert!(is_visible(&g, 0, 0));
        assert!(is_visible(&g, 0, 1));
        assert!(is_visible(&g, 0, 2));
        assert!(is_visible(&g, 0, 3));
        assert!(is_visible(&g, 0, 4));
        assert!(is_visible(&g, 1, 4));
        assert!(is_visible(&g, 2, 4));
        assert!(is_visible(&g, 3, 4));
        assert!(is_visible(&g, 4, 4));
        assert!(is_visible(&g, 4, 3));
        assert!(is_visible(&g, 4, 2));
        assert!(is_visible(&g, 4, 1));
        assert!(is_visible(&g, 4, 0));
        assert!(is_visible(&g, 3, 0));
        assert!(is_visible(&g, 2, 0));
        assert!(is_visible(&g, 1, 0));

        assert!(is_visible(&g, 1, 1));
        assert!(is_visible(&g, 2, 1));
        assert!(!is_visible(&g, 3, 1));

        assert!(is_visible(&g, 1, 2));
        assert!(!is_visible(&g, 2, 2));
        assert!(is_visible(&g, 3, 2));

        assert!(!is_visible(&g, 1, 3));
        assert!(is_visible(&g, 2, 3));
        assert!(!is_visible(&g, 3, 3));
    }

    #[test]
    fn test_count_visible() {
        let g = Grid::from_str(sample());
        assert_eq!(count_visible(&g), 21);
    }

    #[test]
    fn test_viewing_distance() {
        let g = Grid::from_str(sample());

        assert_eq!(viewing_distance(&g, Direction::Up, 0, 0), 0);
        assert_eq!(viewing_distance(&g, Direction::Left, 0, 0), 0);
        assert_eq!(viewing_distance(&g, Direction::Right, 0, 0), 2);
        assert_eq!(viewing_distance(&g, Direction::Down, 0, 0), 2);

        assert_eq!(viewing_distance(&g, Direction::Up, 2, 1), 1);
        assert_eq!(viewing_distance(&g, Direction::Left, 2, 1), 1);
        assert_eq!(viewing_distance(&g, Direction::Right, 2, 1), 2);
        assert_eq!(viewing_distance(&g, Direction::Down, 2, 1), 2);

        assert_eq!(viewing_distance(&g, Direction::Up, 2, 3), 2);
        assert_eq!(viewing_distance(&g, Direction::Left, 2, 3), 2);
        assert_eq!(viewing_distance(&g, Direction::Right, 2, 3), 2);
        assert_eq!(viewing_distance(&g, Direction::Down, 2, 3), 1);
    }

    #[test]
    fn test_empty_range() {
        assert_eq!((0..0).count(), 0);
        assert_eq!((2..2).count(), 0);
        assert_eq!((2..4).count(), 2);
    }

    #[test]
    fn test_scenic_score() {
        let g = Grid::from_str(sample());

        assert_eq!(scening_score(&g, 0, 0), 0);
        assert_eq!(scening_score(&g, 2, 1), 4);
        assert_eq!(scening_score(&g, 2, 3), 8);
    }

    #[test]
    fn test_largest_scenic_score() {
        let g = Grid::from_str(sample());
        assert_eq!(largest_scening_score(&g), 8);
    }
}
