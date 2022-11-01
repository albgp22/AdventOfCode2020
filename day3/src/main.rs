use std::io::{self, BufRead};

fn trees(lines: &Vec<String>, r: usize, d: usize) -> usize {
    lines
        .iter()
        .enumerate()
        .filter(|(i, _)| i % d == 0)
        .filter(|(i, s)| s.chars().nth(i/d * r % s.len()).unwrap() == '#')
        .count()
}

fn main() {
    let lines = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .collect();
    let steps: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    //steps.iter().for_each(|(i,j)| println!("{}", trees(&lines, *i, *j)));
    println!(
        "{}",
        steps
            .iter()
            .map(|(i, j)| trees(&lines, *i, *j))
            .reduce(|x, y| x * y)
            .unwrap()
    );
}
