use std::collections::HashSet;

use crate::utils::read_lines;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Cell {
    x: i32,
    y: i32,
}

pub fn problem1() {
    let mut tail_positions = HashSet::<Cell>::new();
    let mut head_pos = Cell { x: 0, y: 0 };
    let mut tail_pos = head_pos.clone();
    tail_positions.insert(tail_pos.clone());

    for line in read_lines("./src/day9_input") {
        let parts: Vec<&str> = line.split(" ").collect();
        let dir: &str = parts[0];
        let count: u32 = parts[1].parse().expect("not a number");

        for _i in 0..count {
            let next_head_pos: Cell = match dir {
                "U" => Cell { x: head_pos.x, y: head_pos.y + 1 },
                "D" => Cell { x: head_pos.x, y: head_pos.y - 1 },
                "R" => Cell { x: head_pos.x + 1, y: head_pos.y },
                "L" => Cell { x: head_pos.x - 1, y: head_pos.y },
                _ => {
                    unimplemented!("unrecognized dir {}", dir);
                }
            };

            if (tail_pos.x - next_head_pos.x).abs() > 1 || (tail_pos.y - next_head_pos.y).abs() > 1 {
                tail_pos = head_pos.clone();
                tail_positions.insert(tail_pos.clone());
            }

            head_pos = next_head_pos;
        }
    }

    println!("{}", tail_positions.len());
}
