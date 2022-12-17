use std::cmp::Ordering;

use crate::utils::read_lines;

#[derive(PartialEq, Debug, Clone)]
enum Packet {
    Value(i32),
    List(Vec<Packet>)
}

fn read_packet(line: &String) -> Packet {
    let mut stack: Vec<Vec<Packet>> = Vec::new();

    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        match c {
            '[' => {
                stack.push(Vec::new());
            },
            ']' => {
                if let Some(vec) = stack.pop() {
                    if stack.len() == 0 {
                        return Packet::List(vec);
                    }
                    let n = stack.len() - 1;
                    stack[n].push(Packet::List(vec));
                }
            },
            _ => {
                let mut buff = String::new();
                while let Ok(_) = chars[i].to_string().parse::<i32>() {
                    buff.push(chars[i]);
                    i += 1;
                }

                if buff.len() > 0 {
                    let n = stack.len() - 1;
                    stack[n].push(Packet::Value(buff.parse::<i32>().unwrap()));
                    i -= 1;
                }
            }
        }
        i += 1;
    }

    panic!();
}

fn read_packets() -> Vec<Packet> {
    let mut packets: Vec<Packet> = Vec::new();
    for line in read_lines("./src/day13_input") {
        if !line.is_empty() {
            packets.push(read_packet(&line));
        }
    }
    packets
}

fn compare_lists(left: Vec<&Packet>, right: Vec<&Packet>) -> Ordering {
    for i in 0..left.len().max(right.len()) {
        if i >= left.len() {
            return Ordering::Less;
        }

        if i >= right.len() {
            return Ordering::Greater;
        }

        let cmp = compare(left[i], right[i]);
        if !matches!(cmp, Ordering::Equal) {
            return cmp;
        }
    }

    Ordering::Equal
}

fn compare(left: &Packet, right: &Packet) -> Ordering {
    match (left, right) {
        (Packet::Value(v_left), Packet::Value(v_right)) => {
            v_left.cmp(v_right)
        },
        (Packet::Value(_), Packet::List(vs_right)) => {
            compare_lists(vec![left], vs_right.iter().collect())
        },
        (Packet::List(vs_left), Packet::Value(_)) => {
            compare_lists(vs_left.iter().collect(), vec![right])
        },
        (Packet::List(vs_left), Packet::List(vs_right)) => {
            compare_lists(vs_left.iter().collect(), vs_right.iter().collect())
        }
    }
}

fn into_pairs(packets: Vec<Packet>) -> Vec<(Packet, Packet)> {
    let mut pairs = Vec::<(Packet, Packet)>::new();
    let mut buffer = Vec::<Packet>::new();

    for packet in packets.into_iter() {
        buffer.push(packet);

        if buffer.len() == 2 {
            let right = buffer.pop().unwrap();
            let left = buffer.pop().unwrap();
            buffer = Vec::new();
            pairs.push((left, right));
        }
    }
    pairs
}

#[allow(dead_code)]
pub fn problem1() {
    let packets = read_packets();
    let pairs = into_pairs(packets);

    let mut i = 0;
    let mut sum = 0;
    for pair in pairs {
        i += 1;
        sum += match compare(&pair.0, &pair.1) {
            Ordering::Less => i,
            _ => 0
        };
    }

    println!("{}", sum);
}

#[allow(dead_code)]
pub fn problem2() {
    let mut packets = read_packets();

    let divider_1 = Packet::List(vec![Packet::List(vec![Packet::Value(2)])]);
    let divider_2 = Packet::List(vec![Packet::List(vec![Packet::Value(6)])]);

    packets.push(divider_1.clone());
    packets.push(divider_2.clone());
    packets.sort_by(|left, right| compare(left, right));

    let i = packets.iter().position(|x| x == &divider_1).unwrap() + 1;
    let j = packets.iter().position(|x| x == &divider_2).unwrap() + 1;
    println!("{}", i * j);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_packet() {
        let packet = read_packet(&"[1,2,3]".to_string());
        assert_eq!(packet, Packet::List(vec![Packet::Value(1), Packet::Value(2), Packet::Value(3)]));
    }

    #[test]
    fn test_read_nested_packet() {
        let packet = read_packet(&"[[1],[2,3]]".to_string());
        assert_eq!(packet, Packet::List(vec![
            Packet::List(vec![Packet::Value(1)]),
            Packet::List(vec![Packet::Value(2), Packet::Value(3)])
        ]));
    }

    #[test]
    fn test_read_double_digit_packet() {
        let packet = read_packet(&"[10,[10]]".to_string());
        assert_eq!(packet, Packet::List(vec![
            Packet::Value(10),
            Packet::List(vec![Packet::Value(10)])
        ]));
    }
}
