use std::{env, fs};
use std::collections::VecDeque;

#[derive(PartialEq, Debug)]
enum Operand {
    Old,
    Number(usize),
}

#[derive(PartialEq, Debug)]
enum Operation {
    Add(Operand, Operand),
    Mul(Operand, Operand),
}

#[derive(PartialEq, Debug)]
struct Monkey {
    items: VecDeque<usize>,
    formula: Operation,
    divisor: usize,
    target_monkey: (usize, usize),
    items_inspected: usize,
}

impl Monkey {
    fn from_str(s: &str) -> Monkey {
        assert!(s.starts_with("Monkey"));
        let lines = s.lines().skip(1);

        let mut items: Option<VecDeque<usize>> = None;
        let mut formula: Option<Operation> = None;
        let mut divisor: Option<usize> = None;
        let mut target_if_true: Option<usize> = None;
        let mut target_if_false: Option<usize> = None;

        for line in lines {
            match line.trim().split_once(": ") {
                Some(("Starting items", it)) => items = Some(Monkey::parse_items(it)),
                Some(("Operation", op)) => formula = Some(Monkey::parse_operation(op)),
                Some(("Test", t)) => divisor = Some(Monkey::parse_divisor(t)),
                Some(("If true", m)) => target_if_true = Some(Monkey::parse_throw_to(m)),
                Some(("If false", m)) => target_if_false = Some(Monkey::parse_throw_to(m)),
                _ => unreachable!()
            }
        }

        Monkey{
            items: items.unwrap(),
            formula: formula.unwrap(),
            divisor: divisor.unwrap(),
            target_monkey: (target_if_false.unwrap(), target_if_true.unwrap()),
            items_inspected: 0
        }
    }

    fn parse_items(s: &str) -> VecDeque<usize> {
        s.split(", ").map(|x| x.parse::<usize>().expect("cannot parse number")).collect()
    }

    fn parse_operation(s: &str) -> Operation {
        let mut tokens = s.split_whitespace();
        assert_eq!(tokens.next(), Some("new"));
        assert_eq!(tokens.next(), Some("="));
        let lhs = match tokens.next() {
            Some("old") => Operand::Old,
            Some(n) => Operand::Number(n.parse().expect("cannot parse operand")),
            _ => unreachable!(),
        };
        let op = tokens.next();
        let rhs = match tokens.next() {
            Some("old") => Operand::Old,
            Some(n) => Operand::Number(n.parse().expect("cannot parse operand")),
            _ => unreachable!(),
        };

        match op {
            Some("+") => Operation::Add(lhs, rhs),
            Some("*") => Operation::Mul(lhs, rhs),
            _ => unreachable!()
        }
    }

    fn parse_divisor(s: &str) -> usize {
        s.split_whitespace().skip(2).next().expect("invalid test string").parse().expect("cannot parse divisor")
    }

    fn parse_throw_to(s: &str) -> usize {
        s.split_whitespace().skip(3).next().expect("invalid throw to").parse().expect("cannot parse throw to")
    }

    fn inspect(&mut self, worry_level: usize) -> (usize, usize) {
        let level = do_operation(worry_level, &self.formula) / 3;
        let target = if level % self.divisor == 0 {
            self.target_monkey.1
        } else {
            self.target_monkey.0
        };
        self.items_inspected += 1;
        (target, level)
    }

    fn catch_item(&mut self, worry_level: usize) {
        self.items.push_back(worry_level);
    }
}

fn simulate_round(monkeys: &mut Vec<Monkey>) {
    let monkey_count: usize = monkeys.len();
    for i in 0..monkey_count {
        while !monkeys[i].items.is_empty() {
            let level_before = monkeys[i].items.pop_front().unwrap();
            let (target, level) = monkeys[i].inspect(level_before);
            assert!(target < monkey_count);
            monkeys[target].catch_item(level);
        }
    }
}

fn do_operation(old: usize, operation: &Operation) -> usize {
    match operation {
        Operation::Add(a, b) => do_operand(old, &a) + do_operand(old, &b),
        Operation::Mul(a, b) => do_operand(old, &a) * do_operand(old, &b),
    }
}

fn do_operand(old: usize, operand: &Operand) -> usize {
    match operand {
        Operand::Old => old,
        Operand::Number(n) => *n,
    }
}

fn monkey_business(monkeys: &Vec<Monkey>) -> usize {
    let mut total_business: Vec<usize> = monkeys.iter().map(|m| m.items_inspected).collect();
    total_business.sort_by(|a, b| b.cmp(&a));
    total_business[0] * total_business[1]
}

fn main() {
    let input_path = env::args().skip(1).next().expect("give input");
    let contents = fs::read_to_string(input_path).expect("cannot read input");

    let mut monkeys: Vec<Monkey> = contents.split("\n\n").map(Monkey::from_str).collect();
    for _ in 0..20 {
        simulate_round(&mut monkeys);
    }

    println!("monkey business = {}", monkey_business(&monkeys));
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"
    }

    #[test]
    fn test_parse_monkey() {
        let m = Monkey::from_str("Monkey 42:
  Starting items: 4, 8, 15, 16, 23, 42
  Operation: new = old + 23
  Test: divisible by 5
    If true: throw to monkey 0
    If false: throw to monkey 1");
        assert_eq!(m.items, vec![4, 8, 15, 16, 23, 42]);
        assert_eq!(m.formula, Operation::Add(Operand::Old, Operand::Number(23)));
        assert_eq!(m.divisor, 5);
        assert_eq!(m.target_monkey, (1, 0));
    }

    #[test]
    fn test_parse_sample() {
        let monkeys: Vec<Monkey> = sample().split("\n\n").map(Monkey::from_str).collect();
        assert_eq!(monkeys, vec![
           Monkey{
               items: VecDeque::from([79, 98]),
               formula: Operation::Mul(Operand::Old, Operand::Number(19)),
               divisor: 23,
               target_monkey: (3, 2),
               items_inspected: 0,
           },
           Monkey{
               items: VecDeque::from([54, 65, 75, 74]),
               formula: Operation::Add(Operand::Old, Operand::Number(6)),
               divisor: 19,
               target_monkey: (0, 2),
               items_inspected: 0,
           },
           Monkey{
               items: VecDeque::from([79, 60, 97]),
               formula: Operation::Mul(Operand::Old, Operand::Old),
               divisor: 13,
               target_monkey: (3, 1),
               items_inspected: 0,
           },
           Monkey{
               items: VecDeque::from([74]),
               formula: Operation::Add(Operand::Old, Operand::Number(3)),
               divisor: 17,
               target_monkey: (1, 0),
               items_inspected: 0,
           },
        ]);
    }

    #[test]
    fn test_round() {
        let mut monkeys: Vec<Monkey> = sample().split("\n\n").map(Monkey::from_str).collect();
        simulate_round(&mut monkeys);
        assert_eq!(monkeys[0].items, VecDeque::from([20, 23, 27, 26]));
        assert_eq!(monkeys[1].items, VecDeque::from([2080, 25, 167, 207, 401, 1046]));
        assert_eq!(monkeys[2].items, VecDeque::from([]));
        assert_eq!(monkeys[3].items, VecDeque::from([]));
    }

    #[test]
    fn test_20_rounds() {
        let mut monkeys: Vec<Monkey> = sample().split("\n\n").map(Monkey::from_str).collect();
        for _ in 0..20 {
            simulate_round(&mut monkeys);
        }
        assert_eq!(monkeys[0].items, VecDeque::from([10, 12, 14, 26, 34]));
        assert_eq!(monkeys[1].items, VecDeque::from([245, 93, 53, 199, 115]));
        assert_eq!(monkeys[2].items, VecDeque::from([]));
        assert_eq!(monkeys[3].items, VecDeque::from([]));

        assert_eq!(monkeys[0].items_inspected, 101);
        assert_eq!(monkeys[1].items_inspected, 95);
        assert_eq!(monkeys[2].items_inspected, 7);
        assert_eq!(monkeys[3].items_inspected, 105);

        assert_eq!(monkey_business(&monkeys), 10605);
    }

}
