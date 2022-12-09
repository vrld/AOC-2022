use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    let contents = fs::read_to_string(input_path).expect("Cannot read file");

    let mut cals: Vec<i32> = Vec::new();
    let mut current_cals: i32 = 0;

    for line in contents.lines() {
        if line == "" {
            cals.push(current_cals);
            current_cals = 0;
        } else {
            current_cals += line.parse::<i32>().unwrap();
        }
    }
    cals.push(current_cals);
    cals.sort();

    let a = cals.pop().unwrap();
    let b = cals.pop().unwrap();
    let c = cals.pop().unwrap();
    println!("{} + {} + {} = {}", a, b, c, a+b+c);
}
