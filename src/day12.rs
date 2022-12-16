use std::collections::HashMap;

use crate::utils::read_lines;

const ASCII_CODE_A: i32 = 'a' as i32;

type Cell = (i32, i32);

struct Grid {
    heights: Vec<Vec<i32>>,
    start_pos: Cell,
    end_pos: Cell,
}

impl Grid {
    fn get_height_at(&self, (x, y): &Cell) -> Option<i32> {
        if *y >= 0 && *y < self.heights.len() as i32 {
            let row = &self.heights[*y as usize];
            if *x >= 0 && *x < row.len() as i32 {
                return Some(row[*x as usize]);
            }
        }
        None
    }
}

fn char_to_height(char: char) -> i32 {
    char as i32 - ASCII_CODE_A
}

fn read_grid() -> Grid {
    let mut heights: Vec<Vec<i32>> = Vec::new();
    let mut start_pos: Option<Cell> = None;
    let mut end_pos: Option<Cell> = None;
    let mut y = 0;

    for line in read_lines("./src/day12_input") {
        let mut height_row: Vec<i32> = Vec::new();
        let mut x = 0;

        for char in line.chars() {
            let height = match char {
                'S' => {
                    start_pos = Some((x, y));
                    char_to_height('a')
                }
                'E' => {
                    end_pos = Some((x, y));
                    char_to_height('z')
                }
                _ => {
                    char_to_height(char)
                }
            };

            height_row.push(height);
            x += 1;
        }

        heights.push(height_row);
        y += 1;
    }

    Grid {
        heights,
        start_pos: start_pos.expect("start pos not found"),
        end_pos: end_pos.expect("end pos not found"),
    }
}

struct Entry {
    prev: Option<Cell>,
    score: i32,
}

fn backtrack(end_pos: &Cell, visited: &HashMap<Cell, Entry>) -> i32 {
    let mut current: Option<&Cell> = Some(end_pos);
    let mut count = 0;
    let mut vec: Vec<&Cell> = Vec::new();

    while let Some(cell) = current {
        let entry = visited.get(cell).unwrap();
        current = entry.prev.as_ref();
        vec.push(cell);
        count += 1;
    }

    count - 1
}

const ADJACENT_CELLS: [Cell; 4] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1)
];

fn traverse(grid: &Grid, start_pos: &Cell, compare_heights: fn(prev: i32, next: i32) -> bool) -> HashMap<Cell, Entry> {
    let mut visited: HashMap<Cell, Entry> = HashMap::new();
    let mut stack: Vec<Cell> = Vec::new();
    stack.push(*start_pos);
    visited.insert(*start_pos, Entry { prev: None, score: 0 });

    while let Some(pos) = stack.pop() {
        let entry = visited.get(&pos).unwrap();
        let height = grid.get_height_at(&pos).unwrap();
        let score = entry.score + 1;

        for (x, y) in ADJACENT_CELLS {
            let next_pos: Cell = (pos.0 + x, pos.1 + y);

            if let Some(next_height) = grid.get_height_at(&next_pos) {
                if compare_heights(height, next_height) {
                    let prev_score: i32 = match visited.get(&next_pos) {
                        Some(e) => e.score,
                        None => i32::MAX,
                    };

                    if score < prev_score {
                        visited.insert(
                            next_pos,
                            Entry {
                                prev: Some(pos),
                                score,
                            },
                        );

                        stack.push(next_pos);
                    }
                }
            }
        }
    }
    visited
}

fn compare_heights_problem_1(height_a: i32, height_b: i32) -> bool {
    height_b <= height_a + 1
}

#[allow(dead_code)]
pub fn problem1() {
    let grid: Grid = read_grid();
    let visited: HashMap<Cell, Entry> = traverse(&grid, &grid.start_pos, compare_heights_problem_1);
    let steps: i32 = backtrack(&grid.end_pos, &visited);
    println!("{}", steps);
}

fn compare_heights_problem_2(height_a: i32, height_b: i32) -> bool {
    height_b + 1 >= height_a
}

#[allow(dead_code)]
pub fn problem2() {
    let grid: Grid = read_grid();
    let visited: HashMap<Cell, Entry> = traverse(&grid, &grid.end_pos, compare_heights_problem_2);

    let mut y = 0;
    let mut min_steps = i32::MAX;

    for heights_row in grid.heights {
        let mut x = 0;
        for height in heights_row {
            let cell: Cell = (x, y);
            if height == 0 && visited.contains_key(&cell) {
                let steps: i32 = backtrack(&cell, &visited);
                if steps < min_steps {
                    min_steps = steps;
                }
            }
            x += 1;
        }
        y += 1;
    }

    println!("{}", min_steps);
}
