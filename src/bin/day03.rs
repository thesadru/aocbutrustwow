extern crate regex;

fn part1() -> i32 {
    let contents = std::fs::read_to_string("input/day03.txt").unwrap();

    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let result: i32 = re
        .captures_iter(&contents)
        .filter_map(|c| {
            Some(c.get(1)?.as_str().parse::<i32>().ok()? * c.get(2)?.as_str().parse::<i32>().ok()?)
        })
        .sum();

    result
}

enum Instruction {
    DO,
    DONT,
    MUL(i32, i32),
}

fn part2() -> i32 {
    let contents = std::fs::read_to_string("input/day03.txt").unwrap();

    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let instructions: Vec<Instruction> = re
        .captures_iter(&contents)
        .filter_map(|c| match c.get(0)?.as_str().split_once("(")?.0 {
            "mul" => Some(Instruction::MUL(
                c.get(1)?.as_str().parse::<i32>().ok()?,
                c.get(2)?.as_str().parse::<i32>().ok()?,
            )),
            "do" => Some(Instruction::DO),
            "don't" => Some(Instruction::DONT),
            _ => None,
        })
        .collect();

    let mut result = 0;

    let mut enabled = true;
    for instruction in instructions.iter() {
        match instruction {
            Instruction::DO => enabled = true,
            Instruction::DONT => enabled = false,
            Instruction::MUL(a, b) => {
                if enabled {
                    result += a * b
                }
            }
        }
    }

    result
}

fn main() {
    println!("part 1: {}", part1());
    println!("part 2: {}", part2());
}
