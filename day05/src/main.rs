use std::{env, fs};

#[derive(Debug)]
struct Board {
    stacks: Vec<Vec<char>>,
}

fn main() {
    let input_path: String = env::args().skip(1).next().expect("No input file gvien");
    let contents = fs::read_to_string(input_path).expect("Error reading file");

    let mut line_iter = contents.lines().into_iter();

    // parse stack
    let mut stack_description: Vec<&str> = Vec::new();
    loop {
        let line = line_iter.next().expect("input ended unexpectedly");
        if line == "" {
            break;
        }
        stack_description.push(line);
    }

    stack_description.reverse();
    let mut board_one = Board::parse(&stack_description);
    let mut board_two = Board::parse(&stack_description);

    for line in line_iter {
        let (count, from, to) = parse_line(line);
        board_one.move_items(count, from, to);
        board_two.move_items_in_order(count, from, to);
    }

    println!("CraneMover 9000");
    board_one.show();
    board_one.show_message();
    println!();

    println!("CraneMover 9001");
    board_two.show();
    board_two.show_message();
}

impl Board {
    fn parse(lines: &Vec<&str>) -> Board {
        let stack_count: usize = lines[0].trim().split(" ").last().expect("cannot parse stack count").parse().unwrap();

        let mut board = Board{ stacks: Vec::new() };
        board.stacks.resize(stack_count, Vec::<char>::new());

        for line in &lines[1..] {
            let mut chars = line.chars().skip(1).step_by(4);
            for i in 0..stack_count {
                match chars.next() {
                    Some(' ') => (),
                    Some(c) => board.stacks[i].push(c),
                    _ => ()
                }
            }
        }

        board
    }

    fn move_items(&mut self, count: usize, from: usize, to: usize) {
        assert!(0 < from && from <= self.stacks.len());
        assert!(0 < to && to <= self.stacks.len());

        for _ in 0..count {
            if let Some(c) = self.stacks[from-1].pop() {
                self.stacks[to-1].push(c);
            } else {
                panic!("Stack {} is empty", from);
            }
        }
    }

    fn move_items_in_order(&mut self, count: usize, from: usize, to: usize) {
        assert!(0 < from && from <= self.stacks.len());
        assert!(0 < to && to <= self.stacks.len());

        let from_size = self.stacks[from-1].len();
        assert!(from_size >= count);
        let mut substack = self.stacks[from-1].split_off(from_size - count);
        self.stacks[to-1].append(&mut substack);
    }

    fn show(&self) {
        let mut i = 0;
        for s in &self.stacks {
            i += 1;
            print!("{}: ", i);
            for c in s {
                print!("{}", c);
            }
            println!("")
        }
    }

    fn show_message(&self) {
        for s in &self.stacks {
            print!("{}", s.last().unwrap());
        }
        println!("");
    }
}

fn parse_line(line: &str) -> (usize, usize, usize) {
    let tokens: Vec<&str> = line.split(" ").skip(1).step_by(2).collect();
    assert!(tokens.len() == 3);
    let count: usize = tokens[0].parse().unwrap();
    let from: usize = tokens[1].parse().unwrap();
    let to: usize = tokens[2].parse().unwrap();
    (count, from, to)
}
