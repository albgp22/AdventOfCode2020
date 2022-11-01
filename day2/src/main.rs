use std::io::{self, BufRead};

fn is_valid_old_policy(s: &str) -> bool {
    let parts: Vec<&str> = s.split(":").map(|l| l.trim()).collect();
    let pattern = parts[0];
    let password = parts[1];

    let pattern_part: Vec<&str> = pattern.split(" ").collect();
    let range = pattern_part[0];
    let character = pattern_part[1];

    let range_part: Vec<usize> = range.split("-").map(|v| v.parse().unwrap()).collect();
    let occurrences = password.matches(character).count();
    return occurrences >= range_part[0] && occurrences <= range_part[1];
}

fn is_valid_new_policy(s: &str) -> bool {
    let parts: Vec<&str> = s.split(":").map(|l| l.trim()).collect();
    let pattern = parts[0];
    let password = parts[1];

    let pattern_part: Vec<&str> = pattern.split(" ").collect();
    let range = pattern_part[0];
    let character: char = pattern_part[1].parse().unwrap();

    let range_part: Vec<usize> = range.split("-").map(|v| v.parse().unwrap()).collect();
    if password.chars().nth(range_part[0] - 1).unwrap()
        == password.chars().nth(range_part[1] - 1).unwrap()
    {
        return false;
    }
    for i in 0..2 {
        if let Some(c) = password.chars().nth(range_part[i] - 1) {
            if c == character {
                println!("{} match {}", password, pattern);
                return true;
            }
        }
    }
    println!("{} doesn't match {}", password, pattern);
    false
}

fn main() {
    let valid_passwds = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .filter(|l| is_valid_new_policy(l))
        .count();
    println!("{}", valid_passwds);
}
