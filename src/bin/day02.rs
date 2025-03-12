use std::io::prelude::*;

fn is_safe(numbers: &Vec<i32>) -> bool {
    let increasing: bool = numbers[0] < numbers[1];
    for (a, b) in numbers.iter().zip(numbers.iter().skip(1)) {
        let d = (a - b).abs();
        if !((1 <= d && d <= 3) && ((a < b && increasing) || (a > b && !increasing))) {
            return false;
        }
    }

    return true;
}

fn part1() -> i32 {
    let file = std::fs::File::open("input/day02.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let mut safe_count: i32 = 0;

    for line in reader.lines() {
        let numbers: Vec<i32> = line
            .unwrap()
            .split_whitespace()
            .map(|num| num.trim().parse::<i32>().unwrap())
            .collect();

        if is_safe(&numbers) {
            safe_count += 1;
        }
    }

    safe_count
}

fn part2() -> i32 {
    let file = std::fs::File::open("input/day02.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let mut safe_count: i32 = 0;

    for line in reader.lines() {
        let numbers: Vec<i32> = line
            .unwrap()
            .split_whitespace()
            .map(|num| num.trim().parse::<i32>().unwrap())
            .collect();

        for skip in 0..numbers.len() {
            let modified_numbers: Vec<i32> = numbers
                .iter()
                .enumerate()
                .filter_map(|(i, x)| if i == skip { None } else { Some(*x) })
                .collect();

            if is_safe(&modified_numbers) {
                safe_count += 1;
                break;
            }
        }
    }

    safe_count
}

fn main() {
    println!("part 1: {}", part1());
    println!("part 2: {}", part2());
}
