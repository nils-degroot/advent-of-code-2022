use std::{cmp::Ordering, vec};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Number(u32),
}

fn parse_list<S: ToString>(input: S) -> (String, Packet) {
    let input = input.to_string();

    if input.chars().next().unwrap() == '[' {
        let mut input = input[1..].to_string();
        let mut acc = vec![];

        while input.chars().next().unwrap_or(']') != ']' {
            if input.chars().next().unwrap() == ',' {
                input = input[1..].to_string();
            }

            let (remainder, token) = parse_list(input);
            input = remainder;
            acc.push(token);
        }

        let remainder = if input.len() == 0 {
            "".to_string()
        } else {
            input[1..].to_string()
        };

        (remainder, Packet::List(acc))
    } else {
        let first_comma = input.find(',');
        let first_square = input.find(']');
        let end = match (first_comma, first_square) {
            (None, None) => input.len(),
            (None, Some(v)) | (Some(v), None) => v,
            (Some(c), Some(s)) => *vec![c, s].iter().min().unwrap(),
        };

        (
            input[end..].to_string(),
            Packet::Number(input[0..end].parse().unwrap()),
        )
    }
}

fn order_correct(left: &Packet, right: &Packet) -> bool {
    match (left, right) {
        (Packet::List(l), Packet::List(r)) => {
            for (l, r) in l.iter().zip(r.iter()) {
                if !order_correct(l, r) {
                    return false;
                } else if l != r {
                    return true;
                }
            }

            l.len() <= r.len()
        }
        (Packet::List(_), Packet::Number(_)) => {
            order_correct(left, &Packet::List(vec![right.clone()]))
        }
        (Packet::Number(_), Packet::List(_)) => {
            order_correct(&Packet::List(vec![left.clone()]), right)
        }
        (Packet::Number(l), Packet::Number(r)) => l <= r,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let parts = input
        .split("\n\n")
        .map(|p| {
            let mut lines = p.lines();

            (
                parse_list(lines.next().unwrap()).1,
                parse_list(lines.next().unwrap()).1,
            )
        })
        .collect::<Vec<_>>();

    let result = parts
        .iter()
        .enumerate()
        .filter_map(|(i, (l, r))| {
            let order_correct = order_correct(l, r);

            if order_correct {
                Some(i + 1)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    Some(result.iter().sum::<usize>().try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut parts = input
        .lines()
        .filter_map(|p| {
            if p.is_empty() {
                None
            } else {
                Some(parse_list(p).1)
            }
        })
        .collect::<Vec<_>>();

    parts.push(Packet::List(vec![Packet::List(vec![Packet::Number(2)])]));
    parts.push(Packet::List(vec![Packet::List(vec![Packet::Number(6)])]));

    parts.sort_by(|l, r| {
        if l == r {
            Ordering::Equal
        } else if order_correct(l, r) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let divider_1 = parts
        .iter()
        .position(|p| p == &Packet::List(vec![Packet::List(vec![Packet::Number(2)])]))
        .unwrap()
        + 1;
    let divider_2 = parts
        .iter()
        .position(|p| p == &Packet::List(vec![Packet::List(vec![Packet::Number(6)])]))
        .unwrap()
        + 1;

    Some((divider_1 * divider_2).try_into().unwrap())
}

fn main() {
    let input = &aoc::read_file("inputs", 13);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
