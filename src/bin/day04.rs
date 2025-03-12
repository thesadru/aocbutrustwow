extern crate regex;

fn part1() -> i32 {
    let contents = std::fs::read_to_string("input/day04.txt").unwrap();
    let letters: Vec<Vec<char>> = contents
        .split_whitespace()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    let directions: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut found = 0;

    for (dx, dy) in directions.iter() {
        for y in 0..letters.len() {
            for x in 0..letters[y].len() {
                let mut valid = true;
                for (index, char) in "XMAS".char_indices() {
                    let cury = (y as i32) + dy * (index as i32);
                    let curx = (x as i32) + dx * (index as i32);

                    if cury < 0
                        || cury >= (letters.len() as i32)
                        || curx < 0
                        || curx >= (letters[cury as usize].len() as i32)
                        || letters[cury as usize][curx as usize] != char
                    {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    found += 1;
                }
            }
        }
    }

    found
}

fn part2() -> i32 {
    let contents = std::fs::read_to_string("input/day04.txt").unwrap();
    let letters: Vec<Vec<char>> = contents
        .split_whitespace()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    let possible_offsets = [[(-1, -1), (1, 1)], [(-1, 1), (1, -1)]];

    let mut found = 0;

    for y in 0..letters.len() {
        for x in 0..letters[y].len() {
            if letters[y][x] != 'A' {
                continue;
            }

            let mut found_in_x = 0;
            for offsets in possible_offsets.iter() {
                let mut found_letters: Vec<char> = vec![];
                for (dy, dx) in offsets {
                    let cury = (y as i32) + dy;
                    let curx = (x as i32) + dx;

                    if !(cury < 0
                        || (cury as usize) >= letters.len()
                        || curx < 0
                        || (curx as usize) >= letters[cury as usize].len())
                    {
                        found_letters.push(letters[cury as usize][curx as usize]);
                    }
                }
                if found_letters.contains(&'M') && found_letters.contains(&'S') {
                    found_in_x += 1;
                }
            }
            if found_in_x == 2 {
                found += 1;
            }
        }
    }

    found
}

fn main() {
    println!("part 1: {}", part1());
    println!("part 2: {}", part2());
}
