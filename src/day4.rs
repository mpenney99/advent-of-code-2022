use crate::utils::*;

struct Range {
    min: u32,
    max: u32,
}

fn parse_range(str: &str) -> Option<Range> {
    let parts: Vec<&str> = str.split("-").collect();
    let min = parts.get(0).and_then(|s| str::parse::<u32>(s).ok())?;
    let max = parts.get(1).and_then(|s| str::parse::<u32>(s).ok())?;
    Some(Range { min, max })
}

fn fully_overlaps(a: &Range, b: &Range) -> bool {
    (a.min <= b.min && a.max >= b.max) || (b.min <= a.min && b.max >= a.max)
}

fn partially_overlaps(a: &Range, b: &Range) -> bool {
    a.min <= b.max && a.max >= b.min
}

fn problem(overlaps: fn(&Range, &Range) -> bool) {
    let mut result: u32 = 0;

    read_lines("./src/day4_input").for_each(|line| {
        let parts: Vec<&str> = line.split(",").collect();
        let range_a: Option<Range> = parts.get(0).and_then(|s| parse_range(s));
        let Some(range_a) = range_a else { return; };

        let range_b: Option<Range> = parts.get(1).and_then(|s| parse_range(s));
        let Some(range_b) = range_b else { return; };

        if overlaps(&range_a, &range_b) {
            result += 1;
        }
    });

    println!("{}", result);
}

#[allow(dead_code)]
pub fn problem1() {
    problem(fully_overlaps);
}

#[allow(dead_code)]
pub fn problem2() {
    problem(partially_overlaps);
}
