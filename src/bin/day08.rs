fn part1() -> i64 {
    let contents = std::fs::read_to_string("input/day08.txt").unwrap();

    let antennas = contents
        .lines()
        .enumerate()
        .map(|(i, l)| l.chars().enumerate().map(move |(j, x)| ((i as i32, j as i32), x)))
        .flatten()
        .collect::<Vec<_>>();

    let mut antinodes: std::collections::HashSet<(i32, i32)> = std::collections::HashSet::new();

    for (a_pos, a_char) in antennas.iter() {
        if *a_char == '.' {
            continue;
        }
        for (b_pos, b_char) in antennas.iter() {
            if a_char != b_char || a_pos == b_pos {
                continue;
            }
            let antinode_pos = (a_pos.0 + 2 * (b_pos.0 - a_pos.0), a_pos.1 + 2 * (b_pos.1 - a_pos.1));

            if 0 <= antinode_pos.0 && antinode_pos.0 < 50 && 0 <= antinode_pos.1 && antinode_pos.1 < 50 {
                antinodes.insert((antinode_pos.0, antinode_pos.1));
            }
        }
    }
    let antinode_count = antinodes.len() as i64;

    antinode_count
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

fn part2() -> i64 {
    let contents = std::fs::read_to_string("input/day08.txt").unwrap();

    let antennas = contents
        .lines()
        .enumerate()
        .map(|(i, l)| l.chars().enumerate().map(move |(j, x)| ((i as i32, j as i32), x)))
        .flatten()
        .collect::<Vec<_>>();

    let mut antinodes: std::collections::HashSet<(i32, i32)> = std::collections::HashSet::new();

    for (a_pos, a_char) in antennas.iter() {
        if *a_char == '.' {
            continue;
        }
        for (b_pos, b_char) in antennas.iter() {
            if a_char != b_char || a_pos == b_pos {
                continue;
            }
            let large_delta = ((a_pos.0 - b_pos.0), (a_pos.1 - b_pos.1));
            let gcd_delta = gcd(large_delta.0.abs() as u64, large_delta.1.abs() as u64) as i32;
            let delta = (large_delta.0 / gcd_delta, large_delta.1 / gcd_delta);

            let mut i = 0;
            loop {
                let antinode_pos = (a_pos.0 + delta.0 * i, a_pos.1 + delta.1 * i);
                if 0 <= antinode_pos.0 && antinode_pos.0 < 50 && 0 <= antinode_pos.1 && antinode_pos.1 < 50 {
                    antinodes.insert((antinode_pos.0, antinode_pos.1));
                } else {
                    break;
                }
                i += 1;
            }
        }
    }
    let antinode_count = antinodes.len() as i64;

    antinode_count
}

fn main() {
    println!("part 1: {}", part1());
    println!("part 2: {}", part2());
}
