use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Monkey<F: Fn(usize) -> usize> {
    id: u32,
    items: Vec<usize>,
    operation: F,
    test_value: usize,
    test_success: u32,
    test_failure: u32,
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut monkeys = input
        .split("\n\n")
        .map(|p| {
            let mut lines = p.lines();

            let id = lines.next().unwrap();
            let id = id[7..(id.find(|c| c == ':').unwrap())]
                .parse::<u32>()
                .unwrap();

            let items = lines.next().unwrap();
            let items = items[18..]
                .split(", ")
                .map(|i| i.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let mut op_parts = lines.next().unwrap()[19..].split(" ");

            let lhs = op_parts.next().unwrap();
            let op = op_parts.next().unwrap();
            let rhs = op_parts.next().unwrap();

            let operation = move |old: usize| -> usize {
                let lhs = match lhs {
                    "old" => old,
                    n => n.parse().unwrap(),
                };
                let rhs = match rhs {
                    "old" => old,
                    n => n.parse().unwrap(),
                };

                match op {
                    "+" => lhs + rhs,
                    "*" => lhs * rhs,
                    _ => panic!(),
                }
            };

            let test_value = lines.next().unwrap()[21..].parse::<usize>().unwrap();
            let test_success = lines.next().unwrap()[29..].parse::<u32>().unwrap();
            let test_failure = lines.next().unwrap()[30..].parse::<u32>().unwrap();

            Monkey {
                id,
                items,
                operation,
                test_value,
                test_success,
                test_failure,
            }
        })
        .collect::<Vec<_>>();

    let mut inspection = HashMap::new();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let mut monkey = monkeys.clone();
            let monkey = monkey.get_mut(i).unwrap();

            for item in &monkey.items {
                inspection
                    .entry(monkey.id)
                    .and_modify(|v| *v += 1)
                    .or_insert(1);

                let value = ((monkey.operation)(item.clone()) as f64 / 3.0).floor() as usize;
                let idx = if value % monkey.test_value == 0 {
                    monkey.test_success
                } else {
                    monkey.test_failure
                };

                monkeys.get_mut(idx as usize).unwrap().items.push(value);
            }

            monkeys.get_mut(i).unwrap().items.clear();
        }
    }

    let mut values = inspection.values().collect::<Vec<_>>();
    values.sort();
    values.reverse();

    Some(*values.get(0).unwrap() * *values.get(1).unwrap())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut monkeys = input
        .split("\n\n")
        .map(|p| {
            let mut lines = p.lines();

            let id = lines.next().unwrap();
            let id = id[7..(id.find(|c| c == ':').unwrap())]
                .parse::<u32>()
                .unwrap();

            let items = lines.next().unwrap();
            let items = items[18..]
                .split(", ")
                .map(|i| i.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let mut op_parts = lines.next().unwrap()[19..].split(" ");

            let lhs = op_parts.next().unwrap();
            let op = op_parts.next().unwrap();
            let rhs = op_parts.next().unwrap();

            let operation = move |old: usize| -> usize {
                let lhs = match lhs {
                    "old" => old,
                    n => n.parse().unwrap(),
                };
                let rhs = match rhs {
                    "old" => old,
                    n => n.parse().unwrap(),
                };

                match op {
                    "+" => lhs + rhs,
                    "*" => lhs * rhs,
                    _ => panic!(),
                }
            };

            let test_value = lines.next().unwrap()[21..].parse::<usize>().unwrap();
            let test_success = lines.next().unwrap()[29..].parse::<u32>().unwrap();
            let test_failure = lines.next().unwrap()[30..].parse::<u32>().unwrap();

            Monkey {
                id,
                items,
                operation,
                test_value,
                test_success,
                test_failure,
            }
        })
        .collect::<Vec<_>>();

    let mut inspection = HashMap::new();
    let modifier = monkeys.iter().map(|m| m.test_value).product::<usize>();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let mut monkey = monkeys.clone();
            let monkey = monkey.get_mut(i).unwrap();

            for item in &monkey.items {
                inspection
                    .entry(monkey.id)
                    .and_modify(|v| *v += 1)
                    .or_insert(1);

                let value = (monkey.operation)(item.clone() % modifier);
                let idx = if value % monkey.test_value == 0 {
                    monkey.test_success
                } else {
                    monkey.test_failure
                };

                monkeys.get_mut(idx as usize).unwrap().items.push(value);
            }

            monkeys.get_mut(i).unwrap().items.clear();
        }
    }

    let mut values = inspection.values().collect::<Vec<_>>();
    values.sort();
    values.reverse();

    Some(*values.get(0).unwrap() * *values.get(1).unwrap())
}

fn main() {
    let input = &aoc::read_file("inputs", 11);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
