fn is_update_valid(update: &Vec<i32>, rules: &std::collections::HashMap<i32, Vec<i32>>) -> bool {
    let mut before_pages = std::collections::HashSet::<i32>::new();
    for page in update.iter() {
        for rule in rules[page].iter() {
            if before_pages.contains(rule) {
                return false;
            }
        }
        before_pages.insert(*page);
    }

    return true;
}

fn part1() -> i32 {
    let contents = std::fs::read_to_string("input/day05.txt").unwrap();

    let (rule_contents, update_contents) = contents.split_once("\r\n\r\n").unwrap();
    let rules: std::collections::HashMap<i32, Vec<i32>> = rule_contents
        .lines()
        .map(|x| x.split_once("|").unwrap())
        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .fold(
            std::collections::HashMap::<i32, Vec<i32>>::new(),
            |mut m, (a, b)| {
                m.entry(a).or_insert(vec![]).push(b);
                m
            },
        );
    let updates: Vec<Vec<i32>> = update_contents
        .lines()
        .map(|x| x.split(",").map(|x| x.parse::<i32>().unwrap()).collect())
        .collect();

    let mut result = 0;

    for update in updates.iter() {
        if is_update_valid(&update, &rules) {
            result += update[update.len() / 2];
        }
    }

    result
}

fn part2() -> i32 {
    let contents = std::fs::read_to_string("input/day05.txt").unwrap();

    let (rule_contents, update_contents) = contents.split_once("\r\n\r\n").unwrap();
    let rules: std::collections::HashMap<i32, Vec<i32>> = rule_contents
        .lines()
        .map(|x| x.split_once("|").unwrap())
        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .fold(
            std::collections::HashMap::<i32, Vec<i32>>::new(),
            |mut m, (a, b)| {
                m.entry(a).or_insert(vec![]).push(b);
                m
            },
        );
    let updates: Vec<Vec<i32>> = update_contents
        .lines()
        .map(|x| x.split(",").map(|x| x.parse::<i32>().unwrap()).collect())
        .collect();

        let mut result = 0;

    for update in updates.iter() {
        let mut fix_update = update.clone();
        fix_update.sort_by(|a, b| {
            if rules[a].contains(b) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        if *update != fix_update {
            result += fix_update[fix_update.len() / 2];
        }
    }

    result
}

fn main() {
    println!("part 1: {}", part1());
    println!("part 2: {}", part2());
}
