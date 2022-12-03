use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    str::Chars,
};

fn main() {
    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two())
}

fn part_one() -> u32 {
    let lines = read_lines(Path::new("day3.txt")).unwrap();
    let mut my_score = 0;
    for line in lines {
        my_score += find_shared_letter(line)
    }
    return my_score;
}

fn part_two() -> u32 {
    let lines = read_lines(Path::new("day3.txt")).unwrap();
    let mut my_score = 0;
    for i in 0..lines.len() - 2 {
        if i % 3 == 0 {
            let badge_group = [lines[i].chars(), lines[i + 1].chars(), lines[i + 2].chars()];
            my_score += find_badge(badge_group)
        }
    }
    return my_score;
}

fn letters_to_numbers(letter: char) -> u32 {
    let alphabet = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    let mut score = 0;
    if letter.is_uppercase() {
        score += 26
    }
    for i in 0..26 {
        if letter.to_ascii_lowercase() == alphabet[i] {
            score += i + 1;
            break;
        }
    }
    return score.try_into().unwrap();
}

fn find_shared_letter(line: String) -> u32 {
    let length = line.len();
    let mut shared_letter = '0';
    line[..length / 2].chars().for_each(|first_char| {
        line[length / 2..].chars().for_each(|second_char| {
            if first_char == second_char {
                shared_letter = first_char
            }
        });
    });
    return letters_to_numbers(shared_letter);
}

fn find_badge(badge_group: [Chars<'_>; 3]) -> u32 {
    let mut shared_letter = '0';
    badge_group[0].as_str().chars().for_each(|first_char| {
        badge_group[1].as_str().chars().for_each(|second_char| {
            if first_char == second_char {
                if badge_group[2].as_str().contains(first_char) {
                    shared_letter = first_char
                }
            }
        })
    });
    return letters_to_numbers(shared_letter);
}

fn read_lines(path: &Path) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    return reader.lines().collect();
}
