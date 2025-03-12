fn find_initial_guard(matrix: &Vec<Vec<char>>) -> (i32, i32) {
    for (row_idx, row) in matrix.iter().enumerate() {
        for (col_idx, &c) in row.iter().enumerate() {
            if c == '^' {
                return (row_idx as i32, col_idx as i32);
            }
        }
    }

    panic!();
}

fn part1() -> i32 {
    let contents = std::fs::read_to_string("input/day06.txt").unwrap();
    let letters: Vec<Vec<char>> = contents
        .split_whitespace()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    let mut visited = std::collections::HashSet::<(i32, i32)>::new();
    let mut cur_pos = find_initial_guard(&letters);
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut cur_dir = 0;

    loop {
        visited.insert(cur_pos);

        let next_pos = (cur_pos.0 + directions[cur_dir].0, cur_pos.1 + directions[cur_dir].1);
        if next_pos.0 < 0
            || (next_pos.0 as usize) >= letters.len()
            || next_pos.1 < 0
            || (next_pos.1 as usize) >= letters[next_pos.0 as usize].len()
        {
            break;
        }
        if letters[next_pos.0 as usize][next_pos.1 as usize] == '#' {
            cur_dir = (cur_dir + 1) % 4;
        } else {
            cur_pos = next_pos;
        }
    }

    visited.len() as i32
}

fn part2() -> i32 {
    let contents = std::fs::read_to_string("input/day06.txt").unwrap();
    let letters: Vec<Vec<char>> = contents
        .split_whitespace()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let initial_guard = find_initial_guard(&letters);
    let mut infinite_loop_count = 0;
    for sub_y in 0..letters.len() {
        for sub_x in 0..letters[sub_y].len() {
            if letters[sub_y][sub_x] != '.' {
                continue;
            }

            let mut is_infinite_loop = false;
            let mut visited = std::collections::HashSet::<(i32, i32, usize)>::new();
            let mut cur_pos = initial_guard;
            let mut cur_dir = 0;

            loop {
                let next_pos = (cur_pos.0 + directions[cur_dir].0, cur_pos.1 + directions[cur_dir].1);
                if visited.contains(&(next_pos.0, next_pos.1, cur_dir)) {
                    is_infinite_loop = true;
                    break;
                }
                if next_pos.0 < 0
                    || (next_pos.0 as usize) >= letters.len()
                    || next_pos.1 < 0
                    || (next_pos.1 as usize) >= letters[next_pos.0 as usize].len()
                {
                    break;
                }
                if letters[next_pos.0 as usize][next_pos.1 as usize] == '#'
                    || ((next_pos.0 as usize) == sub_y && (next_pos.1 as usize) == sub_x)
                {
                    cur_dir = (cur_dir + 1) % 4;
                } else {
                    cur_pos = next_pos;
                    visited.insert((cur_pos.0, cur_pos.1, cur_dir));
                }
                
            }
            if is_infinite_loop {
                infinite_loop_count += 1;
            }
        }
    }

    infinite_loop_count
}

fn main() {
    println!("part 1: {}", part1());
    println!("part 2: {}", part2());
}
