extern crate itertools;

use itertools::Itertools;

fn part1() -> i64 {
    let contents = std::fs::read_to_string("input/day07.txt").unwrap();
    let calibration: Vec<(i64, Vec<i64>)> = contents
        .lines()
        .map(|x| {
            let (rawtvalue, rawnumbers) = x.split_once(": ").unwrap();
            let numbers = rawnumbers
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            (rawtvalue.parse::<i64>().unwrap(), numbers)
        })
        .collect();


    let mut result = 0;
    for (testvalue, numbers) in calibration.iter() {
        let mut canreach = false;
        for operators in std::iter::repeat_n("+*".chars(), numbers.len() - 1).multi_cartesian_product() {
            let mut current_value = numbers[0];
            for (op, n) in operators.iter().zip(numbers.iter().skip(1)) {
                match op {
                    '+' => current_value += n,
                    '*' => current_value *= n,
                    _ => panic!(),
                }
            }
            if &current_value == testvalue {
                canreach = true;
                break;
            }
        }
        if canreach {
            result += testvalue;
        }
    }

    result
}

fn part2() -> i64 {
    let contents = std::fs::read_to_string("input/day07.txt").unwrap();
    let calibration: Vec<(i64, Vec<i64>)> = contents
        .lines()
        .map(|x| {
            let (rawtvalue, rawnumbers) = x.split_once(": ").unwrap();
            let numbers = rawnumbers
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            (rawtvalue.parse::<i64>().unwrap(), numbers)
        })
        .collect();


    let mut result = 0;
    for (testvalue, numbers) in calibration.iter() {
        let mut canreach = false;
        for operators in std::iter::repeat_n("+*|".chars(), numbers.len() - 1).multi_cartesian_product() {
            let mut current_value = numbers[0];
            for (op, n) in operators.iter().zip(numbers.iter().skip(1)) {
                match op {
                    '+' => current_value += n,
                    '*' => current_value *= n,
                    '|' => current_value = (current_value.to_string() + &n.to_string()).parse::<i64>().unwrap(),
                    _ => panic!(),
                }
            }
            if &current_value == testvalue {
                canreach = true;
                break;
            }
        }
        if canreach {
            result += testvalue;
        }
    }

    result
}

fn main() {
    println!("part 1: {}", part1());
    println!("part 2: {}", part2());
}
