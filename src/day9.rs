use std::collections::HashSet;

use crate::utils::read_lines;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Cell {
    x: i32,
    y: i32,
}

fn problem(tail_size: usize) -> usize {
    let mut visited_cells = HashSet::<Cell>::new();
    let mut head = Cell { x: 0, y: 0 };
    let mut tail: Vec<Cell> = (0..tail_size).map(|_| head.clone()).collect();
    visited_cells.insert(tail[tail.len() - 1].clone());

    for line in read_lines("./src/day9_input") {
        let parts: Vec<&str> = line.split(" ").collect();
        let dir: &str = parts[0];
        let count: u32 = parts[1].parse().expect("not a number");

        for _ in 0..count {
            head = match dir {
                "U" => Cell { x: head.x, y: head.y + 1 },
                "D" => Cell { x: head.x, y: head.y - 1 },
                "R" => Cell { x: head.x + 1, y: head.y },
                "L" => Cell { x: head.x - 1, y: head.y },
                _ => {
                    unimplemented!("unrecognized dir {}", dir);
                }
            };

            let mut prev: &Cell = &head;

            for i in 0..tail_size {
                let tail_cell = &tail[i];
                let diff_x: i32 = prev.x - tail_cell.x;
                let diff_y: i32 = prev.y - tail_cell.y;

                if diff_x.abs() > 1 || diff_y.abs() > 1 {
                    tail[i] = Cell {
                        x: tail_cell.x + diff_x.signum(),
                        y: tail_cell.y + diff_y.signum()
                    };
                }
                
                prev = &tail[i];
            }

            visited_cells.insert(tail[tail.len() - 1].clone());
        }
    }

    visited_cells.len()
}

#[allow(dead_code)]
pub fn problem1() {
    println!("{}", problem(1));
}

#[allow(dead_code)]
pub fn problem2() {
    println!("{}", problem(9));
}
