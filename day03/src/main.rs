use std::{env, fs, str};
use std::collections::HashSet;


fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    let contents = fs::read_to_string(input_path).expect("Error reading file");

    let mut total_line_prio = 0;
    let mut total_group_prio = 0;
    let mut group_lines: Vec<String> = Vec::new();
    for line in contents.lines() {
        total_line_prio += line_prio(line.as_bytes());

        group_lines.push(line.to_string());
        if group_lines.len() >= 3 {
            total_group_prio += group_prio(&group_lines);
            group_lines.clear();
        }
    }
    println!("total_line_prio: {}", total_line_prio);
    println!("total_group_prio: {}", total_group_prio);
}

fn line_prio(line: &[u8]) -> i32 {
    assert!(line.len() % 2 == 0);
    let a: HashSet<&u8> = HashSet::from_iter(&line[..line.len()/2]);
    let b: HashSet<&u8> = HashSet::from_iter(&line[line.len()/2..]);
    let common = a.intersection(&b);
    common.into_iter().map(|c| item_priority(**c)).reduce(|a, i| a + i).unwrap_or(0)
}

fn group_prio(group: &Vec<String>) -> i32 {
    assert!(group.len() == 3);
    let a: HashSet<&u8> = HashSet::from_iter(group[0].as_bytes());
    let b: HashSet<&u8> = HashSet::from_iter(group[1].as_bytes());
    let c: HashSet<&u8> = HashSet::from_iter(group[2].as_bytes());
    let common = a.intersection(&b);

    let mut res = 0;
    for x in c.iter() {
        match common.clone().into_iter().find(|&q| q == x) {
            Some(_) => res += item_priority(**x),
            None => ()
        }
    }
    res
}

fn item_priority(item: u8) -> i32 {
    match item {
        b'a' ..= b'z' => (item - b'a' + 1).into(),
        b'A' ..= b'Z' => (item - b'A' + 27).into(),
        _ => 0
    }
}
