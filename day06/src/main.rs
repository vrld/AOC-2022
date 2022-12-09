use std::{env, fs};
use std::collections::VecDeque;

fn main() {
    let input_path = env::args().skip(1).next().expect("give input file");
    let contents = fs::read_to_string(input_path).expect("no read file");
    for line in contents.lines() {
        let short_marker_pos = find_marker_pos(&line, 4);
        println!("4 marker at {}", short_marker_pos + 1);

        let long_marker_pos = find_marker_pos(&line, 14);
        println!("14 marker at {}", long_marker_pos + 1);
    }
}

fn find_marker_pos(line: &str, sequence_length: usize) -> u32 {
    let mut deque: VecDeque<char> = VecDeque::with_capacity(sequence_length);
    for (i, c) in line.chars().enumerate() {
        deque.push_back(c);
        deque.make_contiguous();

        if deque.len() >= sequence_length {
            if all_distinct(&deque) {
                return i as u32
            }
            deque.pop_front();
        }
    }
    panic!("No starting sequence");
}

fn all_distinct(v: &VecDeque<char>) -> bool {
    let (s, _) = v.as_slices();
    for i in 1..s.len() {
        if s[0..i].contains(&s[i]) {
            return false;
        }
    }
    true
}
