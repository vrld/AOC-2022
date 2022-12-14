use std::{env, fs};
use std::str::Chars;
use std::cmp::Ordering;
use std::iter::Peekable;

fn main() {
    let input_path = env::args().skip(1).next().expect("no input");
    let contents = fs::read_to_string(input_path).expect("cannot read");
    let pairs = parse_pairs(&contents).expect("parsing failed");

    println!("sum of indices of pairs in right order: {}", sum_indices_in_right_order(&pairs));
    println!("decoder_key: {}", decoder_key(&pairs));
}

#[derive(Debug, PartialEq)]
enum Pkg {
    List(Vec<Pkg>),
    Num(u32),
}

#[derive(Debug, PartialEq)]
enum ParserError {
    InvalidCharacter(char),
    UnexpectedEndOfInput,
    NotAPairOfPackets(String),
}

fn parse_pairs(s: &str) -> Result<Vec<(Pkg, Pkg)>, ParserError> {
    let mut res: Vec<(Pkg, Pkg)> = vec![];
    for block in s.split_terminator("\n\n") {
        match block.split_once("\n") {
            None => return Err(ParserError::NotAPairOfPackets(block.into())),
            Some((a, b)) => res.push((parse_line(a)?, parse_line(b)?)),
        }
    }

    Ok(res)
}

fn parse_line(line: &str) -> Result<Pkg, ParserError> {
    // I would use a JSON deserializer, but where is the fun in that?
    let mut chars = line.chars().peekable();
    parse_packet(&mut chars)
}

fn parse_packet(chars: &mut Peekable<Chars<'_>>) -> Result<Pkg, ParserError> {
    match chars.peek() {
        Some('[') => parse_list(chars),
        Some('0') | Some('1') | Some('2') | Some('3') | Some('4') | Some('5') | Some('6') | Some('7') | Some('8') | Some('9') => parse_number(chars),
        Some(c) => Err(ParserError::InvalidCharacter(*c)),
        None => Err(ParserError::UnexpectedEndOfInput),
    }
}

fn parse_number(chars: &mut Peekable<Chars<'_>>) -> Result<Pkg, ParserError> {
    let mut digits: Vec<char> = vec![];
    loop {
        match chars.peek() {
            Some('0') | Some('1') | Some('2') | Some('3') | Some('4') | Some('5') | Some('6') | Some('7') | Some('8') | Some('9') => {
                digits.push(chars.next().unwrap())
            },
            Some(']') | Some(',') => break,
            Some(c) => return Err(ParserError::InvalidCharacter(*c)),
            None => return Err(ParserError::UnexpectedEndOfInput),
        }
    }

    if digits.len() > 0 {
        let s: String = digits.into_iter().collect();
        Ok(Pkg::Num(s.parse().unwrap()))
    } else {
        let c = chars.peek().unwrap_or(&' ');
        Err(ParserError::InvalidCharacter(*c))
    }
}

fn parse_list(chars: &mut Peekable<Chars<'_>>) -> Result<Pkg, ParserError> {
    match chars.next() {
        Some('[') => (),
        Some(c) => return Err(ParserError::InvalidCharacter(c)),
        None => return Err(ParserError::UnexpectedEndOfInput),
    }

    let mut res: Vec<Pkg> = vec![];
    'outer: loop {
        'inner: loop {
            match chars.peek() {
                Some(' ') | Some('\t') | Some(',') | Some(']') => (),
                _ => break 'inner,
            }
            match chars.next() {
                Some(']') => break 'outer,
                _ => (),
            }
        }
        match parse_packet(chars) {
            Ok(p) => res.push(p),
            e => return e,
        }
    }

    Ok(Pkg::List(res))
}

#[derive(Debug, PartialEq)]
enum PkgOrder {
    Correct,
    Undecided,
    Incorrect,
}

fn pair_in_right_order((a, b): (&Pkg, &Pkg)) -> PkgOrder {
    match (a, b) {
        (Pkg::Num(x), Pkg::Num(y)) => {
            if x < y {
                PkgOrder::Correct
            } else if x > y {
                PkgOrder::Incorrect
            } else {
                PkgOrder::Undecided
            }
        },
        (Pkg::List(first), Pkg::List(second)) => {
            for (i, x) in first.iter().enumerate() {
                if let Some(y) = second.get(i) {
                    match pair_in_right_order((&x, &y)) {
                        PkgOrder::Undecided => (),
                        order => return order,
                    }
                } else {
                    // second ran out of items first
                    return PkgOrder::Incorrect;
                }
            }
            if first.len() < second.len() {
                // first ran out of items
                return PkgOrder::Correct;
            } else {
                return PkgOrder::Undecided;
            }
        },
        (x, Pkg::Num(y)) => pair_in_right_order((x, &Pkg::List(vec![Pkg::Num(*y)]))),
        (Pkg::Num(x), y) => pair_in_right_order((&Pkg::List(vec![Pkg::Num(*x)]), y)),
    }
}

fn sum_indices_in_right_order(pairs: &Vec<(Pkg, Pkg)>) -> usize {
    pairs.iter()
        .enumerate()
        .filter(|(_, p)| pair_in_right_order((&p.0, &p.1)) == PkgOrder::Correct)
        .map(|(i, _)| i + 1)
        .sum()
}

fn decoder_key(pairs: &Vec<(Pkg, Pkg)>) -> usize {
    let dividers = (
        Pkg::List(vec![Pkg::List(vec![Pkg::Num(2)])]),
        Pkg::List(vec![Pkg::List(vec![Pkg::Num(6)])]),
    );

    let mut packages: Vec<&Pkg> = vec![&dividers.0, &dividers.1];
    for (p, q) in pairs {
        packages.push(p);
        packages.push(q);
    }
    packages.sort_by(|p, q| as_ordering(&pair_in_right_order((p, q))));

    packages.iter().enumerate()
        .filter(|(_, p)| p == &&&dividers.0 || p == &&&dividers.1)
        .map(|(i, _)| i + 1)
        .product()
}

fn as_ordering(o: &PkgOrder) -> Ordering {
    match o {
        PkgOrder::Correct => Ordering::Less,
        PkgOrder::Undecided => Ordering::Equal,
        PkgOrder::Incorrect => Ordering::Greater,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> &'static str {
        "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"
    }

    #[test]
    fn test_parse_number() {
        let t1: &str = "123,";
        assert_eq!(parse_number(&mut t1.chars().peekable()), Ok(Pkg::Num(123)));

        let t2: &str = ",123,";
        assert_eq!(parse_number(&mut t2.chars().peekable()), Err(ParserError::InvalidCharacter(',')));

        let t3: &str = "abc";
        assert_eq!(parse_number(&mut t3.chars().peekable()), Err(ParserError::InvalidCharacter('a')));

        let t4: &str = "1b3";
        assert_eq!(parse_number(&mut t4.chars().peekable()), Err(ParserError::InvalidCharacter('b')));

        let t5: &str = "";
        assert_eq!(parse_number(&mut t5.chars().peekable()), Err(ParserError::UnexpectedEndOfInput));
    }

    #[test]
    fn test_parse_list() {
        let t1: &str = "[]";
        assert_eq!(parse_list(&mut t1.chars().peekable()), Ok(Pkg::List(vec![])));

        let t2: &str = "[1]";
        assert_eq!(parse_list(&mut t2.chars().peekable()), Ok(Pkg::List(vec![Pkg::Num(1)])));

        let t3: &str = "[1, 2]";
        assert_eq!(parse_list(&mut t3.chars().peekable()), Ok(Pkg::List(vec![Pkg::Num(1), Pkg::Num(2)])));

        let t4: &str = "[1, [2, 3]]";
        assert_eq!(parse_list(&mut t4.chars().peekable()),
                   Ok(Pkg::List(vec![Pkg::Num(1), Pkg::List(vec![Pkg::Num(2), Pkg::Num(3)])])));

        let t5: &str = "[1";
        assert_eq!(parse_list(&mut t5.chars().peekable()), Err(ParserError::UnexpectedEndOfInput));

        let t6: &str = "[1, ,";
        assert_eq!(parse_list(&mut t6.chars().peekable()), Err(ParserError::UnexpectedEndOfInput));

        let t7: &str = "[1, [2,]";
        assert_eq!(parse_list(&mut t7.chars().peekable()), Err(ParserError::UnexpectedEndOfInput));

        let t8: &str = "[1, [2]";
        assert_eq!(parse_list(&mut t8.chars().peekable()), Err(ParserError::UnexpectedEndOfInput));
    }

    #[test]
    fn test_parse_line() {
        let t1: &str = "[]";
        assert_eq!(parse_line(&t1), Ok(Pkg::List(vec![])));

        let t2: &str = "[1, [2, 3]]";
        assert_eq!(parse_line(&t2),
                   Ok(Pkg::List(vec![Pkg::Num(1), Pkg::List(vec![Pkg::Num(2), Pkg::Num(3)])])));
    }

    #[test]
    fn test_parse_sample() {
        let s = parse_pairs(sample()).expect("parsing failed");

        assert_eq!(s[0],
                   (Pkg::List(vec![Pkg::Num(1), Pkg::Num(1), Pkg::Num(3), Pkg::Num(1), Pkg::Num(1)]),
                    Pkg::List(vec![Pkg::Num(1), Pkg::Num(1), Pkg::Num(5), Pkg::Num(1), Pkg::Num(1)])));

        assert_eq!(s[1],
                   (Pkg::List(vec![Pkg::List(vec![Pkg::Num(1)]), Pkg::List(vec![Pkg::Num(2), Pkg::Num(3), Pkg::Num(4)])]),
                    Pkg::List(vec![Pkg::List(vec![Pkg::Num(1)]), Pkg::Num(4)])));

        assert_eq!(s[2],
                   (Pkg::List(vec![Pkg::Num(9)]),
                    Pkg::List(vec![Pkg::List(vec![Pkg::Num(8), Pkg::Num(7), Pkg::Num(6)])])));

        assert_eq!(s[3],
                   (Pkg::List(vec![Pkg::List(vec![Pkg::Num(4), Pkg::Num(4)]), Pkg::Num(4), Pkg::Num(4)]),
                    Pkg::List(vec![Pkg::List(vec![Pkg::Num(4), Pkg::Num(4)]), Pkg::Num(4), Pkg::Num(4), Pkg::Num(4)])));

        assert_eq!(s[4],
                   (Pkg::List(vec![Pkg::Num(7), Pkg::Num(7), Pkg::Num(7), Pkg::Num(7)]),
                    Pkg::List(vec![Pkg::Num(7), Pkg::Num(7), Pkg::Num(7)])));

        assert_eq!(s[5],
                   (Pkg::List(vec![]),
                    Pkg::List(vec![Pkg::Num(3)])));

        assert_eq!(s[6],
                   (Pkg::List(vec![Pkg::List(vec![Pkg::List(vec![])])]),
                    Pkg::List(vec![Pkg::List(vec![])])));

        assert_eq!(s[7],
                   (Pkg::List(vec![Pkg::Num(1), Pkg::List(vec![Pkg::Num(2), Pkg::List(vec![Pkg::Num(3), Pkg::List(vec![Pkg::Num(4), Pkg::List(vec![Pkg::Num(5), Pkg::Num(6), Pkg::Num(7)])])])]), Pkg::Num(8), Pkg::Num(9)]),
                    Pkg::List(vec![Pkg::Num(1), Pkg::List(vec![Pkg::Num(2), Pkg::List(vec![Pkg::Num(3), Pkg::List(vec![Pkg::Num(4), Pkg::List(vec![Pkg::Num(5), Pkg::Num(6), Pkg::Num(0)])])])]), Pkg::Num(8), Pkg::Num(9)])));
    }

    #[test]
    fn test_pair_in_right_order() {
        let s = parse_pairs(sample()).expect("parsing failed");

        assert_eq!(pair_in_right_order((&s[0].0, &s[0].1)), PkgOrder::Correct);
        assert_eq!(pair_in_right_order((&s[1].0, &s[1].1)), PkgOrder::Correct);
        assert_eq!(pair_in_right_order((&s[2].0, &s[2].1)), PkgOrder::Incorrect);
        assert_eq!(pair_in_right_order((&s[3].0, &s[3].1)), PkgOrder::Correct);
        assert_eq!(pair_in_right_order((&s[4].0, &s[4].1)), PkgOrder::Incorrect);
        assert_eq!(pair_in_right_order((&s[5].0, &s[5].1)), PkgOrder::Correct);
        assert_eq!(pair_in_right_order((&s[6].0, &s[6].1)), PkgOrder::Incorrect);
        assert_eq!(pair_in_right_order((&s[7].0, &s[7].1)), PkgOrder::Incorrect);
    }

    #[test]
    fn test_sum_indices_in_right_order() {
        let s = parse_pairs(sample()).expect("parsing failed");
        assert_eq!(sum_indices_in_right_order(&s), 13);
    }

    #[test]
    fn test_decoder_key() {
        let s = parse_pairs(sample()).expect("parsing failed");
        assert_eq!(decoder_key(&s), 140);
    }
}
