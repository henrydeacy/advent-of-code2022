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
    let mut total = 0;
    let lines = read_lines(Path::new("day4.txt")).unwrap();
    for line in lines {
        let ranges = get_ranges(&line);
        let first_min: i32 = ranges[0].parse().unwrap();
        let first_max: i32 = ranges[1].parse().unwrap();
        let second_min: i32 = ranges[2].parse().unwrap();
        let second_max: i32 = ranges[3].parse().unwrap();
        if ((first_min - second_min) * (first_max - second_max)) <= 0 {
            total += 1
        }
    }
    return total;
}

fn part_two() -> u32 {
    let mut total = 0;
    let lines = read_lines(Path::new("day4.txt")).unwrap();
    for line in lines {
        let ranges = get_ranges(&line);
        let first_min: i32 = ranges[0].parse().unwrap();
        let first_max: i32 = ranges[1].parse().unwrap();
        let second_min: i32 = ranges[2].parse().unwrap();
        let second_max: i32 = ranges[3].parse().unwrap();
        if first_min - second_min < 0 {
            if first_max - second_min >= 0 {
                total += 1;
            }
        } else if second_min - first_min < 0 {
            if second_max - first_min >= 0 {
                total += 1
            }
        } else if first_min == second_min || first_max == second_max {
            total += 1;
        }
    }
    return total;
}

fn get_ranges(line: &str) -> Vec<&str> {
    let split = line.split(&[',', '-']);
    let ranges = split.collect::<Vec<&str>>();

    return ranges;
}

fn read_lines(path: &Path) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    return reader.lines().collect();
}
