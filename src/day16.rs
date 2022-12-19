use std::collections::{HashMap, HashSet};

use once_cell::sync::Lazy;
use regex::Regex;

use crate::utils::read_lines;

struct Valve {
    flow_rate: i32,
    tunnels: Vec<String>,
}

fn parse_valve(line: &str) -> (String, Valve) {
    static REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (\w+(?:,\s?\w+)*)").unwrap()
    });

    let captures = REGEX.captures(line).expect(line);
    let name: String = captures.get(1).unwrap().as_str().to_string();
    let flow_rate: i32 = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
    let tunnels_str: &str = captures.get(3).unwrap().as_str();
    let tunnels: Vec<String> = tunnels_str
        .split(",")
        .map(|x| x.trim().to_string())
        .collect();
    
    (name, Valve {  flow_rate, tunnels, })
}

fn parse_valves() -> HashMap<String, Valve> {
    read_lines("./src/day16_input")
        .map(|x| parse_valve(&x))
        .collect()
}

// fn move_down_tunnel(t: i32, name: &str, valves: &HashMap<String, Valve>, closed_valves: &HashSet<String>) -> i32 {
//     if t <= 2 { // is there enough time to move down tunnel + rotate another valve?
//         return 0;
//     }

//     let t = t - 1;
//     let valve = valves.get(name).unwrap();
//     valve.tunnels.iter().map(|next| {
//         solve(t, next, valves, &closed_valves)
//     }).max().unwrap_or(0)
// }

// fn open_valve(t: i32, name: &str, valves: &HashMap<String, Valve>, closed_valves: &HashSet<String>) -> i32 {
//     if t <= 1 || closed_valves.contains(name) {
//         return 0;
//     }

//     let t = t - 1;
//     let valve = valves.get(name).unwrap();
//     let v = valve.flow_rate * t;

//     let mut closed_valves = closed_valves.clone();
//     closed_valves.insert(name.to_string());

//     v + move_down_tunnel(t, name, valves, &closed_valves)
// }

fn solve(t: i32, name: &str, valves: &HashMap<String, Valve>, open_valves: &HashSet<String>) -> i32 {
    if t <= 1 {
        return 0;
    }

    let valve = valves.get(name).unwrap();
    let mut max_value = 0;

    for open_valve in [true, false] {
        if open_valve && (valve.flow_rate == 0 || open_valves.contains(name)) {
            continue;
        }

        let mut t = t;
        let mut v = 0;
        
        let mut open_valves = open_valves.clone();
        if open_valve {
            t -= 1;
            v += valve.flow_rate * t;
            open_valves.insert(name.to_string());
        }
        
        if t >= 2 { // is there enough time to move down tunnel + rotate another valve ?
            t -= 1;

            let mut max_v = 0;
            for tunnel in valve.tunnels.iter() {
                // if !visited.contains(tunnel) {
                    let v = solve(t, tunnel, valves, &open_valves);
                    max_v = max_v.max(v);
                // }
            };

            v += max_v;
        }

        max_value = max_value.max(v);
    }

    max_value
}

pub fn problem1() {
    let valves = parse_valves();
    let t = solve(30, &"AA", &valves, &HashSet::new());
    println!("{}", t);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        problem1();
    }
}
