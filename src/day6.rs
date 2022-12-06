use std::{collections::HashSet, cmp::min};

use crate::utils::*;

pub fn run(marker_length: usize) {
    read_lines("./src/day6_input").for_each(|line| {
        let Ok(text) = line else { return; };

        let chars: Vec<char> = text.chars().collect();
        let mut set: HashSet<char> = HashSet::new();

        for i in 0..chars.len() {
            set.clear();

            for j in i..min(chars.len(), i + marker_length) {
                set.insert(chars[j]);
            }

            if set.len() == marker_length {
                println!("{}", i + marker_length);
                break;
            }
        }
    });
}

pub fn problem1() {
    run(4);
}

pub fn problem2() {
    run(14);
}
