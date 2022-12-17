use std::collections::HashMap;

use crate::utils::read_lines;

enum CellType {
    Rock,
    Sand,
}

type Cell = (i32, i32);

const START_POS: Cell = (500, 0);
const MOVE_OFFSETS: [Cell; 3] = [(0, 1), (-1, 1), (1, 1)];

fn parse_cell(str: &str) -> Cell {
    let parts: Vec<&str> = str.split(",").collect();
    let x = parts[0].parse::<i32>().unwrap();
    let y = parts[1].parse::<i32>().unwrap();
    (x, y)
}

fn read_grid(max_height: &mut i32) -> HashMap<Cell, CellType> {
    let mut grid: HashMap<Cell, CellType> = HashMap::new();
    
    for line in read_lines("./src/day14_input") {
        let mut prev_cell: Option<Cell> = None;

        for cell in line.split(" -> ").map(parse_cell) {
            if let Some(prev) = prev_cell {
                for x in prev.0.min(cell.0)..=prev.0.max(cell.0) {
                    for y in prev.1.min(cell.1)..=prev.1.max(cell.1) {
                        grid.insert((x, y), CellType::Rock);
                        if y > *max_height {
                            *max_height = y;
                        }
                    }
                }
            };

            prev_cell = Some(cell);
        }
    }

    grid
}

fn simulate_sand_particle_1(grid: &mut HashMap<Cell, CellType>, max_height: i32) -> bool {
    let mut prev_pos: Cell = START_POS;
    loop {
        let next_pos = MOVE_OFFSETS.iter()
            .map(|offset| (prev_pos.0 + offset.0, prev_pos.1 + offset.1))
            .filter(|next_pos| !grid.contains_key(&next_pos))
            .next();

        match next_pos {
            Some(next_pos) => {
                prev_pos = next_pos;
                if next_pos.1 > max_height {
                    break false;
                }
            },
            None => {
                grid.insert(prev_pos, CellType::Sand);
                break true;
            }
        }
    }
}

fn simulate_sand_particle_2(grid: &mut HashMap<Cell, CellType>, floor_height: i32) -> bool {
    let mut prev_pos: Cell = START_POS;
    loop {
        let next_pos = MOVE_OFFSETS.iter()
            .map(|offset| (prev_pos.0 + offset.0, prev_pos.1 + offset.1))
            .filter(|next_pos| !grid.contains_key(&next_pos) && next_pos.1 < floor_height)
            .next();

        match next_pos {
            Some(next_pos) => {
                prev_pos = next_pos;
            },
            None => {
                grid.insert(prev_pos, CellType::Sand);
                break prev_pos != START_POS;
            }
        }
    }
}

#[allow(dead_code)]
pub fn problem1() {
    let mut max_height: i32 = 0;
    let mut grid: HashMap<Cell, CellType> = read_grid(&mut max_height);    
    let mut count = 0;
    while simulate_sand_particle_1(&mut grid, max_height) {
        count += 1;
    }
    println!("{}", count);
}

#[allow(dead_code)]
pub fn problem2() {
    let mut max_height: i32 = 0;
    let mut grid: HashMap<Cell, CellType> = read_grid(&mut max_height);
    let mut count = 0;
    println!("{}", max_height);
    while simulate_sand_particle_2(&mut grid, max_height + 2) {
        count += 1;
    }
    println!("{}", count + 1);
}
