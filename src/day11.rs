use std::collections::VecDeque;

struct MonkeyTest {
    div: u64,
    when_true: usize,
    when_false: usize,
}

struct Monkey {
    items: VecDeque<u64>,
    operation: fn(u64) -> u64,
    test: MonkeyTest,
    inspected_count: u64,
}

fn create_monkeys() -> Vec<Monkey> {
    let monkey0 = Monkey {
        items: VecDeque::from([84, 72, 58, 51]),
        operation: |prev| prev * 3,
        test: MonkeyTest {
            div: 13,
            when_true: 1,
            when_false: 7,
        },
        inspected_count: 0,
    };

    let monkey1 = Monkey {
        items: VecDeque::from([88, 58, 58]),
        operation: |prev| prev + 8,
        test: MonkeyTest {
            div: 2,
            when_true: 7,
            when_false: 5,
        },
        inspected_count: 0,
    };

    let monkey2 = Monkey {
        items: VecDeque::from([93, 82, 71, 77, 83, 53, 71, 89]),
        operation: |prev| prev * prev,
        test: MonkeyTest {
            div: 7,
            when_true: 3,
            when_false: 4,
        },
        inspected_count: 0,
    };

    let monkey3 = Monkey {
        items: VecDeque::from([81, 68, 65, 81, 73, 77, 96]),
        operation: |prev| prev + 2,
        test: MonkeyTest {
            div: 17,
            when_true: 4,
            when_false: 6,
        },
        inspected_count: 0,
    };

    let monkey4 = Monkey {
        items: VecDeque::from([75, 80, 50, 73, 88]),
        operation: |prev| prev + 3,
        test: MonkeyTest {
            div: 5,
            when_true: 6,
            when_false: 0,
        },
        inspected_count: 0,
    };

    let monkey5 = Monkey {
        items: VecDeque::from([59, 72, 99, 87, 91, 81]),
        operation: |prev| prev * 17,
        test: MonkeyTest {
            div: 11,
            when_true: 2,
            when_false: 3,
        },
        inspected_count: 0,
    };

    let monkey6 = Monkey {
        items: VecDeque::from([86, 69]),
        operation: |prev| prev + 6,
        test: MonkeyTest {
            div: 3,
            when_true: 1,
            when_false: 0,
        },
        inspected_count: 0,
    };

    let monkey7 = Monkey {
        items: VecDeque::from([91]),
        operation: |prev| prev + 1,
        test: MonkeyTest {
            div: 19,
            when_true: 2,
            when_false: 5,
        },
        inspected_count: 0,
    };

    vec![
        monkey0, monkey1, monkey2, monkey3, monkey4, monkey5, monkey6, monkey7,
    ]
}

fn run_round(monkeys: &mut Vec<Monkey>, factor: u64) {
    for i in 0..monkeys.len() {
        let monkey = &mut monkeys[i];
        let mut items_to_move: Vec<(usize, u64)> = Vec::new();

        while let Some(item) = monkey.items.pop_front() {
            monkey.inspected_count += 1;

            let item = (monkey.operation)(item) % factor;
            
            let next_monkey = if item % monkey.test.div == 0 {
                monkey.test.when_true
            } else {
                monkey.test.when_false
            };

            items_to_move.push((next_monkey, item));
        }

        for (monkey, item) in items_to_move.into_iter() {
            monkeys[monkey].items.push_back(item);
        }
    }
}

#[allow(dead_code)]
pub fn problem1() {
    let mut monkeys: Vec<Monkey> = create_monkeys();
    for _ in 0..20 {
        run_round(&mut monkeys, 3);
    }

    monkeys.sort_by(|m1, m2| m2.inspected_count.cmp(&m1.inspected_count));
    println!("{}", monkeys[0].inspected_count * monkeys[1].inspected_count);
}

#[allow(dead_code)]
pub fn problem2() {
    let mut monkeys: Vec<Monkey> = create_monkeys();
    let factor: u64 = monkeys.iter().fold(1, |acc, m| acc * m.test.div);

    for _ in 0..10_000 {
        run_round(&mut monkeys, factor);
    }

    monkeys.sort_by(|m1, m2| m2.inspected_count.cmp(&m1.inspected_count));
    println!( "{}", monkeys[0].inspected_count * monkeys[1].inspected_count);
}
