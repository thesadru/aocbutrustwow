fn part1() -> i64 {
    let contents = std::fs::read_to_string("input/day24.txt").unwrap();
    let (contents_initial, contents_gates) = contents.split_once("\r\n\r\n").unwrap();

    let mut wires = contents_initial
        .lines()
        .fold(std::collections::HashMap::<&str, bool>::new(), |mut wires, x| {
            let (gate, value) = x.split_once(": ").unwrap();
            wires.insert(gate, value == "1");
            wires
        });
    let gates = contents_gates
        .lines()
        .map(|x| {
            let parts: Vec<&str> = x.split_whitespace().collect();
            (parts[0], parts[1], parts[2], parts[4])
        })
        .collect::<Vec<_>>();

    let mut any_ran = true;
    while any_ran {
        any_ran = false;
        for gate in gates.iter() {
            if wires.contains_key(gate.0) && wires.contains_key(gate.2) && !wires.contains_key(gate.3) {
                any_ran = true;
                match gate.1 {
                    "AND" => wires.insert(gate.3, wires[gate.0] & wires[gate.2]),
                    "OR" => wires.insert(gate.3, wires[gate.0] | wires[gate.2]),
                    "XOR" => wires.insert(gate.3, wires[gate.0] ^ wires[gate.2]),
                    _ => unimplemented!(),
                };
            }
        }
    }

    let result = (0..=45)
        .map(|i| wires[format!("z{:02}", i).as_str()])
        .rev()
        .fold(0, |acc, b| (acc << 1) + (b as i64));
    
    result
}

fn part2() -> i64 {
    let contents = std::fs::read_to_string("input/day24.txt").unwrap();
    let (contents_initial, contents_gates) = contents.split_once("\r\n\r\n").unwrap();

    let mut wires = contents_initial
        .lines()
        .fold(std::collections::HashMap::<&str, bool>::new(), |mut wires, x| {
            let (gate, value) = x.split_once(": ").unwrap();
            wires.insert(gate, value == "1");
            wires
        });
    let mut gates = contents_gates
        .lines()
        .map(|x| {
            let parts: Vec<&str> = x.split_whitespace().collect();
            (parts[0], parts[1], parts[2], parts[4])
        })
        .collect::<Vec<_>>();

    // TODO: finish this

    0
}

fn main() {
    println!("part 1: {}", part1());
    println!("part 2: {}", part2());
}
