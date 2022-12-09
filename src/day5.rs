use once_cell::sync::Lazy;
use regex::Regex;
use std::cell::RefCell;

use crate::utils::*;

#[derive(Debug)]
struct Command {
    amount: usize,
    from: usize,
    to: usize,
}

static RE_COMMAND: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap());

/**
 * Don't judge me...
 * [Q]         [N]             [N]
 * [H]     [B] [D]             [S] [M]
 * [C]     [Q] [J]         [V] [Q] [D]
 * [T]     [S] [Z] [F]     [J] [J] [W]
 * [N] [G] [T] [S] [V]     [B] [C] [C]
 * [S] [B] [R] [W] [D] [J] [Q] [R] [Q]
 * [V] [D] [W] [G] [P] [W] [N] [T] [S]
 * [B] [W] [F] [L] [M] [F] [L] [G] [J]
 * 1   2   3   4   5   6   7   8   9
 */

fn stacks() -> Vec<RefCell<Vec<&'static str>>> {
    vec![
        RefCell::new(vec!["B", "V", "S", "N", "T", "C", "H", "Q"]),
        RefCell::new(vec!["W", "D", "B", "G"]),
        RefCell::new(vec!["F", "W", "R", "T", "S", "Q", "B"]),
        RefCell::new(vec!["L", "G", "W", "S", "Z", "J", "D", "N"]),
        RefCell::new(vec!["M", "P", "D", "V", "F"]),
        RefCell::new(vec!["F", "W", "J"]),
        RefCell::new(vec!["L", "N", "Q", "B", "J", "V"]),
        RefCell::new(vec!["G", "T", "R", "C", "J", "Q", "S", "N"]),
        RefCell::new(vec!["J", "S", "Q", "C", "W", "D", "M"]),
    ]
}

fn read_commands() -> Vec<Command> {
    read_lines("./src/day5_input")
        .map(|line| {
            let captures = RE_COMMAND.captures(&line).expect("invalid command");
            let amount: usize = captures.get(1).unwrap().as_str().parse().unwrap();
            let from: usize = captures.get(2).unwrap().as_str().parse().unwrap();
            let to: usize = captures.get(3).unwrap().as_str().parse().unwrap();
            Command { amount, from, to }
        })
        .collect()
}

fn problem(exec_command: fn(&mut Vec<RefCell<Vec<&str>>>, command: &Command)) {
    let mut stacks = stacks();

    let commands = read_commands();
    for command in commands {
        exec_command(&mut stacks, &command);
    }

    let result = stacks
        .into_iter()
        .map(|stack| stack.borrow().last().unwrap().to_string())
        .collect::<String>();

    println!("{}", result);
}

#[allow(dead_code)]
pub fn problem1() {
    problem(|stacks, command| {
        let mut from_stack = stacks
            .get(command.from - 1)
            .expect("invalid index")
            .borrow_mut();
        let mut to_stack = stacks
            .get(command.to - 1)
            .expect("invalid index")
            .borrow_mut();

        for _ in 0..command.amount {
            if let Some(item) = from_stack.pop() {
                to_stack.push(item);
            }
        }
    });
}

#[allow(dead_code)]
pub fn problem2() {
    problem(|stacks, command| {
        let mut from_stack = stacks
            .get(command.from - 1)
            .expect("invalid index")
            .borrow_mut();
        let mut to_stack = stacks
            .get(command.to - 1)
            .expect("invalid index")
            .borrow_mut();

        let mut items: Vec<&str> = Vec::new();
        for _ in 0..command.amount {
            if let Some(item) = from_stack.pop() {
                items.push(item);
            }
        }

        to_stack.extend(items);
    });
}
