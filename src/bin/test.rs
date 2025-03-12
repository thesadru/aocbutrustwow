fn main() {
    let contents = std::fs::read_to_string("trains.txt").unwrap();
    let (name_contents, distance_contents) = contents.split_once("\n").unwrap();
    let names: Vec<String> = name_contents
        .trim()
        .split("\t")
        .map(|x| x.trim_matches(|c: char| c.is_whitespace() || c == ':').to_string())
        .filter(|x| !x.is_empty())
        .collect();
    let distances: Vec<Vec<i32>> = distance_contents
        .trim()
        .lines()
        .map(|line| line.split("\t").filter_map(|x| x.parse().ok()).collect())
        .collect();
    assert!(names.len() == distances.len());
    dbg!(names, distances);
}
