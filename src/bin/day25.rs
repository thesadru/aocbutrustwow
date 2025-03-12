fn part1() -> i64 {
    let contents = std::fs::read_to_string("input/day25.txt").unwrap();
    let mut locks = vec![];
    let mut keys = vec![];
    for schematic in contents.split("\r\n\r\n") {
        let schematic_chars = schematic.split_whitespace().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        let mut heights: Vec<u8> = vec![0; schematic_chars[0].len()];
        for i in 0..schematic_chars.len() {
            for j in 0..schematic_chars[i].len() {
                if schematic_chars[i][j] == '#' {
                    heights[j] += 1;
                }
            }
        }
        if schematic_chars[0][0] == '#' {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }

    let mut unique_pairs = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            let mut is_valid = true;
            for (a, b) in lock.iter().zip(key) {
                if a+b > 7 {
                    is_valid = false;
                }
            }
            unique_pairs += is_valid as i64;
        }
    }

    unique_pairs
}

fn part2() -> i64 {
    0
}

fn main() {
    println!("part 1: {}", part1());
    println!("part 2: {}", part2());
}
