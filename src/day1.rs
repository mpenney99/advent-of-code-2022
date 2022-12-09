
use crate::utils::*;

#[allow(dead_code)]
pub fn problem2() {
    let mut values: Vec<u32> = Vec::new();
    let mut current: u32 = 0;

    read_lines("./src/day1_input")
        .for_each(|line| {
            if let Ok(value) = line.parse::<u32>() {
                current += value;
            } else {
                values.push(current);
                current = 0;
            }
        });

    values.push(current);

    values.sort();
    
    let result: u32 = values.iter().rev().take(3).sum();
    println!("{}", result);
}
