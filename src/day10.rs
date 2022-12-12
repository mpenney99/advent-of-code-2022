use std::io::{self, Write};

use crate::utils::read_lines;

struct Command {
    value: i32,
    cycles: u32
}

fn parse_command(line: &String) -> Command {
    let parts: Vec<&str> = line.split(" ").collect();
    match parts[0] {
        "addx" => {
            let value: i32 = parts[1].parse().expect("cannot parse number");
            Command { value, cycles: 2 }
        },
        "noop" => {
            Command { value: 0, cycles: 1 }
        },
        _ => panic!("unrecognized command")
    }
}

#[allow(dead_code)]
pub fn problem1() {
    let mut register = 1;
    let mut cycle_count: i32 = 0;
    let mut total: i32 = 0;

    for line in read_lines("./src/day10_input") {
        let mut command = parse_command(&line);

        while command.cycles > 0 {
            cycle_count += 1;
            if (cycle_count - 20) % 40 == 0 {
                total += register * cycle_count;
            }

            command.cycles -= 1;
            if command.cycles == 0 {
                register += command.value;
            }
        }
    }

    println!("{}", total);
}

const SCREEN_WIDTH: usize = 40;

#[allow(dead_code)]
pub fn problem2() {
    let mut x = 1;
    let mut buffer = String::new();
    let mut stdout = io::stdout().lock();

    for line in read_lines("./src/day10_input") {
        let mut command = parse_command(&line);

        while command.cycles > 0 {
            let cx = buffer.len() as i32;
            if (cx - x).abs() <= 1 {
                buffer.push('#');
            } else {
                buffer.push('.');
            }

            if buffer.len() >= SCREEN_WIDTH {
                stdout.write_all(buffer.as_bytes()).unwrap();
                stdout.write_all(b"\n").unwrap();
                buffer = String::new();
            }

            command.cycles -= 1;
            if command.cycles == 0 {
                x += command.value;
            }
        }
    }
    
    stdout.write_all(buffer.as_bytes()).unwrap();
}
