use std::{env, fs};

struct Range {
    from: i32,
    to: i32,
}

impl Range {
    fn from_def(s: &str) -> Range {
        let parts: Vec<&str> = s.split("-").collect();
        assert!(parts.len() == 2);
        Range {
            from: parts[0].parse::<i32>().unwrap(),
            to: parts[1].parse::<i32>().unwrap(),
        }
    }

    fn contains(&self, other: &Range) -> bool {
        self.from <= other.from && self.to >= other.to
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.to >= other.from && self.from <= other.to
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    let contents = fs::read_to_string(input_path).expect("Error reading file");

    let mut total_overlap_count = 0;
    let mut partial_overlap_count = 0;
    for line in contents.lines() {
        let ranges: Vec<&str> = line.split(",").collect();
        assert!(ranges.len() == 2);
        let range_one = Range::from_def(ranges[0]);
        let range_two = Range::from_def(ranges[1]);

        total_overlap_count += count_total_overlaps(&range_one, &range_two);
        partial_overlap_count += count_partial_overlaps(&range_one, &range_two);
    }

    println!("total_overlap_count = {}", total_overlap_count);
    println!("partial_overlap_count = {}", partial_overlap_count);
}

fn count_total_overlaps(a: &Range, b: &Range) -> i32 {
    if a.contains(b) || b.contains(a) {
        1
    } else {
        0
    }
}

fn count_partial_overlaps(a: &Range, b: &Range) -> i32 {
    if a.overlaps(b) {
        1
    } else {
        0
    }
}
