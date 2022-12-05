use once_cell::sync::Lazy;
use regex::Regex;
use std::{fs, io};

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

fn stacks() -> Vec<Vec<&'static str>> {
    vec![
        vec!["B", "V", "S", "N", "T", "C", "H", "Q"],
        vec!["W", "D", "B", "G"],
        vec!["F", "W", "R", "T", "S", "Q", "B"],
        vec!["L", "G", "W", "S", "Z", "J", "D", "N"],
        vec!["M", "P", "D", "V", "F"],
        vec!["F", "W", "J"],
        vec!["L", "N", "Q", "B", "J", "V"],
        vec!["G", "T", "R", "C", "J", "Q", "S", "N"],
        vec!["J", "S", "Q", "C", "W", "D", "M"],
    ]
}

fn parse_commands(it: &mut io::Lines<io::BufReader<fs::File>>) -> Vec<Command> {
    it.map(|line| {
        let text = line.unwrap();
        let Some(captures) = RE_COMMAND.captures(&text) else {
            panic!("invalid command: {}", text);
        };

        let amount: usize = captures.get(1).unwrap().as_str().parse().unwrap();
        let from: usize = captures.get(2).unwrap().as_str().parse().unwrap();
        let to: usize = captures.get(3).unwrap().as_str().parse().unwrap();
        Command { amount, from, to }
    })
    .collect()
}

fn problem(exec_command: fn(&mut Vec<Vec<&str>>, command: &Command)) {
    let mut lines = read_lines("./src/day5_input");
    let mut stacks = stacks();
    let commands = parse_commands(&mut lines);

    for command in commands {
        exec_command(&mut stacks, &command);
    }

    let result = stacks
        .into_iter()
        .map(|stack| stack.last().unwrap().to_string())
        .collect::<String>();

    println!("{}", result);
}

pub fn problem1() {
    problem(|stacks, command| {
        for _ in 0..command.amount {
            if let Some(from_stack) = stacks.get_mut(command.from - 1) {
                if let Some(item) = from_stack.pop() {
                    if let Some(to_stack) = stacks.get_mut(command.to - 1) {
                        to_stack.push(item);
                    }
                }
            }
        }
    });
}

pub fn problem2() {
    problem(|stacks, command| {
        let mut items: Vec<&str> = Vec::new();

        if let Some(from_stack) = stacks.get_mut(command.from - 1) {
            for _ in 0..command.amount {
                if let Some(item) = from_stack.pop() {
                    items.push(item);
                }
            }
        }

        if let Some(to_stack) = stacks.get_mut(command.to - 1) {
            while let Some(item) = items.pop() {
                to_stack.push(item);
            }
        }
    });
}
