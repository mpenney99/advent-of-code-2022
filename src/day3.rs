use std::collections::HashSet;

use crate::utils::*;

const ASCII_CODE_A_LOWER: u32 = 97;
const ASCII_CODE_Z_LOWER: u32 = 122;
const ASCII_CODE_A_UPPER: u32 = 65;
const ASCII_CODE_Z_UPPER: u32 = 90;

fn get_priority(char: char) -> u32 {
    let code = char as u32;
    if code >= ASCII_CODE_A_LOWER && code <= ASCII_CODE_Z_LOWER {
        code - ASCII_CODE_A_LOWER + 1
    } else if code >= ASCII_CODE_A_UPPER && code <= ASCII_CODE_Z_UPPER {
        code - ASCII_CODE_A_UPPER + 27
    } else {
        panic!("invalid char {}", char);
    }
}

pub fn problem1() {
    let mut result: u32 = 0;

    read_lines("./src/day3_input").for_each(|line| {
        if let Ok(text) = line {
            let mid: usize = text.len() / 2;
            let chars_set: HashSet<char> = HashSet::from_iter(text[0..mid].chars());
            let char: Option<char> = text[mid..text.len()].chars()
                .find(|c| { chars_set.contains(c) });

            if let Some(c) = char {
                result += get_priority(c);
            }
        }
    });

    println!("{}", result);
}

pub fn problem2() {
    let mut result: u32 = 0;
    let mut intersect: Option<HashSet<char>> = None;
    let mut i: u32 = 0;

    read_lines("./src/day3_input").for_each(|line| {
        if let Ok(text) = line {
            let mut next_intersect: HashSet<char> = HashSet::new();

            text.chars().for_each(|c| {
                if match &intersect { Some(set) => set.contains(&c), None => true } {
                    next_intersect.insert(c);
                }
            });

            intersect = Some(next_intersect);
            i += 1;

            if i == 3 {
                if let Some(c) = intersect.as_ref().and_then(|set| { set.iter().next() }) {
                    result += get_priority(*c);
                }

                intersect = None;
                i = 0;
            }
        }
    });

    println!("{}", result);
}
