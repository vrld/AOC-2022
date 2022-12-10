use std::{env, fs};

#[derive(Debug, PartialEq)]
enum Operation {
    Noop,
    Addx(i32),
}

#[derive(Debug)]
struct VM {
    register_history: Vec<i32>,
}

impl VM {
    fn new() -> VM {
        VM{ register_history: vec![0, 1] }
    }

    fn register(&self) -> i32 {
        *self.register_history.last().unwrap()
    }

    fn signal_strength_at(&self, cycle: usize) -> i32 {
        assert!(cycle + 1 < self.register_history.len());
        self.register_history[cycle] * cycle as i32
    }

    fn run(&mut self, op: &Operation) {
        match op {
            &Operation::Noop => {
                self.register_history.push(self.register())
            },
            &Operation::Addx(v) => {
                let r = self.register();
                self.register_history.push(r);
                self.register_history.push(r + v)
            }
        }
    }

    fn parse_command(line: &str) -> Operation {
        if line == "noop" {
            Operation::Noop
        } else if line.starts_with("addx") {
            let n: i32 = line.split_whitespace().skip(1)
                .next().expect("missing argument to addx")
                .parse().expect("cannot parse argument");
            Operation::Addx(n)
        } else {
            panic!("Unknown command: {}", line)
        }
    }
}

fn main() {
    let input_path: String = env::args().skip(1).next().expect("give input");
    let contents = fs::read_to_string(input_path).expect("cannot read input");

    let mut vm = VM::new();
    for line in contents.lines() {
        vm.run(&VM::parse_command(line));
    }

    let solution_1: i32 = (0..=220).skip(20).step_by(40).map(|i| vm.signal_strength_at(i)).sum();
    println!("{}", solution_1);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> &'static str {
        "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"
    }

    #[test]
    fn test_parse_command() {
        assert_eq!(VM::parse_command("noop"), Operation::Noop);
        assert_eq!(VM::parse_command("addx 12"), Operation::Addx(12));
        assert_eq!(VM::parse_command("addx 0"), Operation::Addx(0));
        assert_eq!(VM::parse_command("addx -9"), Operation::Addx(-9));
    }

    #[test]
    fn test_operations() {
        let mut vm = VM::new();
        assert_eq!(vm.register(), 1);
        assert_eq!(vm.register_history, vec![0, 1]);

        vm.run(&Operation::Addx(1));
        assert_eq!(vm.register(), 2);
        assert_eq!(vm.register_history, vec![0, 1, 1, 2]);

        vm.run(&Operation::Noop);
        assert_eq!(vm.register(), 2);
        assert_eq!(vm.register_history, vec![0, 1, 1, 2, 2]);

        vm.run(&Operation::Addx(-5));
        assert_eq!(vm.register(), -3);
        assert_eq!(vm.register_history, vec![0, 1, 1, 2, 2, 2, -3]);
    }

    #[test]
    fn test_input() {
        let mut vm = VM::new();
        for line in sample().lines() {
            let cycle_last = vm.register_history.len()-1;
            vm.run(&VM::parse_command(line));
            println!("[{} -- {}] {} -> {}", cycle_last, vm.register_history.len()-1, line, vm.register());
        }
        assert_eq!(vm.register_history[20], 21);
        assert_eq!(vm.register_history[60], 19);
        assert_eq!(vm.register_history[100], 18);
        assert_eq!(vm.register_history[140], 21);
        assert_eq!(vm.register_history[180], 16);
        assert_eq!(vm.register_history[220], 18);

        assert_eq!(vm.signal_strength_at(20), 420);
        assert_eq!(vm.signal_strength_at(60), 1140);
        assert_eq!(vm.signal_strength_at(100), 1800);
        assert_eq!(vm.signal_strength_at(140), 2940);
        assert_eq!(vm.signal_strength_at(180), 2880);
        assert_eq!(vm.signal_strength_at(220), 3960);

        assert_eq!((0..=220).skip(20).step_by(40).map(|i| vm.signal_strength_at(i)).sum::<i32>(), 13140);
    }
}
