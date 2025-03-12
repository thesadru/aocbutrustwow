use std::io::prelude::*;

fn part1() -> i32 {
    let file = std::fs::File::open("input/day01.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    for line in reader.lines() {
        let numbers: Vec<i32> = line
            .unwrap()
            .split_whitespace()
            .map(|num| num.trim().parse::<i32>().unwrap())
            .collect();

        left.push(numbers[0]);
        right.push(numbers[1]);
    }

    left.sort_unstable();
    right.sort_unstable();

    let mut total_difference = 0;
    for (a, b) in left.iter().zip(right.iter()) {
        total_difference += (a - b).abs();
    }

    total_difference
}

fn part2() -> i32 {
    let file = std::fs::File::open("input/day01.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let mut left: Vec<i32> = vec![];
    let mut right: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();

    for line in reader.lines() {
        let numbers: Vec<i32> = line
            .unwrap()
            .split_whitespace()
            .map(|num| num.trim().parse::<i32>().unwrap())
            .collect();

        left.push(numbers[0]);
        *right.entry(numbers[1]).or_insert(0) += 1;
    }

    let mut total_score = 0;
    for a in left.iter() {
        total_score += a * right.get(a).unwrap_or(&0);
    }

    total_score
}

fn main() {
    println!("part 1: {}", part1());
    println!("part 2: {}", part2());
}
