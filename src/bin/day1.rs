use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn main() {
    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two())
}

fn part_one() -> u32 {
    let lines = read_lines(Path::new("day1.txt")).unwrap();
    let mut values_to_compare = [0, 0];
    for line in lines {
        if line.is_empty() {
            values_to_compare.sort();
            values_to_compare[0] = 0;
        } else {
            let n: u32 = line.parse().unwrap();
            values_to_compare[0] += n;
        }
    }
    values_to_compare[1]
}

fn part_two() -> u32 {
    let lines = read_lines(Path::new("day1.txt")).unwrap();
    let mut values_to_compare = [0, 0, 0, 0];
    for line in lines {
        if line.is_empty() {
            values_to_compare.sort();
            values_to_compare[0] = 0;
        } else {
            let n: u32 = line.parse().unwrap();
            values_to_compare[0] += n;
        }
    }
    values_to_compare[1] + values_to_compare[2] + values_to_compare[3]
}

fn read_lines(path: &Path) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    return reader.lines().collect();
}
