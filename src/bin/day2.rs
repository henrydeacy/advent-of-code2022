use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn main() {
    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two())
}

fn part_one() -> i32 {
    let lines = read_lines(Path::new("day2.txt")).unwrap();
    let mut my_score = 0;
    for line in lines {
        let my_choice = choice(line.chars().nth(2).unwrap());
        let their_choice = choice(line.chars().nth(0).unwrap());
        my_score += my_choice;
        if their_choice - my_choice == 2 || my_choice - their_choice == 1 {
            my_score += 6
        }
        if my_choice == their_choice {
            my_score += 3
        }
    }
    return my_score;
}

fn part_two() -> i32 {
    let lines = read_lines(Path::new("day2.txt")).unwrap();
    let mut my_score = 0;
    for line in lines {
        let their_choice = choice(line.chars().nth(0).unwrap());
        let my_choice = choice(line.chars().nth(2).unwrap());
        my_score += play_against(my_choice, their_choice)
    }
    return my_score;
}

fn choice(choice: char) -> i32 {
    if choice == 'A' || choice == 'X' {
        return 1;
    }
    if choice == 'B' || choice == 'Y' {
        return 2;
    }
    if choice == 'C' || choice == 'Z' {
        return 3;
    } else {
        return 0;
    }
}

fn win_against(their_choice: i32) -> i32 {
    let mut score = 6;
    if (their_choice + 1).rem_euclid(3) == 0 {
        score += 3
    } else {
        score += (their_choice + 1).rem_euclid(3);
    }
    return score;
}
fn draw_against(their_choice: i32) -> i32 {
    return their_choice + 3;
}
fn lose_against(their_choice: i32) -> i32 {
    let mut score = 0;
    if their_choice == 1 {
        score += 3
    } else {
        score += their_choice - 1;
    }
    return score.try_into().unwrap();
}

fn play_against(my_choice: i32, their_choice: i32) -> i32 {
    if my_choice == 1 {
        return lose_against(their_choice);
    }
    if my_choice == 2 {
        return draw_against(their_choice);
    }
    if my_choice == 3 {
        return win_against(their_choice);
    }
    return 0;
}

fn read_lines(path: &Path) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    return reader.lines().collect();
}
