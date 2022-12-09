use std::env;
use std::fs;

#[derive(Clone)]
enum RPS {
    Rock,
    Paper,
    Scissor
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    let contents = fs::read_to_string(input_path).expect("Cannot read file");

    let mut total_score = 0;
    for line in contents.lines() {
        total_score += line_score(line);
    }
    println!("{}", total_score);
}

fn line_score(line: &str) -> i32 {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    let opponent = parse_move(&tokens[0]);
    let guide = &tokens[1];
    let m = my_move(&opponent, guide);
    my_score(&m) + move_score(&opponent, &m)
}

fn parse_move(m: &str) -> RPS {
    match m {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissor,
        _ => panic!("Invalid move")
    }
}

fn my_move(opponent: &RPS, guide: &str) -> RPS {
    match (opponent, guide) {
        (_, "X") => losing_move(opponent),
        (_, "Y") => opponent.clone(),
        (_, "Z") => winning_move(opponent),
        _ => panic!("invalid strategy")
    }
}

fn losing_move(m: &RPS) -> RPS {
    match m {
        RPS::Rock => RPS::Scissor,
        RPS::Paper => RPS::Rock,
        RPS::Scissor => RPS::Paper,
    }
}

fn winning_move(m: &RPS) -> RPS {
    match m {
        RPS::Rock => RPS::Paper,
        RPS::Paper => RPS::Scissor,
        RPS::Scissor => RPS::Rock,
    }
}

fn my_score(m: &RPS) -> i32 {
    match m {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissor => 3,
    }
}

fn move_score(a: &RPS, b: &RPS) -> i32 {
    match (a, b) {
        (RPS::Rock, RPS::Rock)
            | (RPS::Paper, RPS::Paper)
            | (RPS::Scissor, RPS::Scissor) => 3,
        (RPS::Rock, RPS::Paper)
            | (RPS::Paper, RPS::Scissor)
            | (RPS::Scissor, RPS::Rock) => 6,
        _ => 0,
    }
}
