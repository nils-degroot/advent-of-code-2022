use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let actions = input
        .lines()
        .map(|l| {
            let mut split = l.split(' ');

            (
                split.next().unwrap().chars().next().unwrap(),
                split.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);

    let mut visited = HashSet::new();
    visited.insert(tail_pos);

    let distance_to_big = |lhs: (i32, i32), rhs: (i32, i32)| {
        ((lhs.0 - rhs.0) as i32).abs() >= 2 || ((lhs.1 - rhs.1) as i32).abs() >= 2
    };

    for action in actions {
        for _ in 0..action.1 {
            head_pos = match action.0 {
                'U' => (head_pos.0 + 1, head_pos.1),
                'D' => (head_pos.0 - 1, head_pos.1),
                'L' => (head_pos.0, head_pos.1 - 1),
                'R' => (head_pos.0, head_pos.1 + 1),
                _ => panic!("Invalid variant"),
            };

            if distance_to_big(head_pos, tail_pos) {
                tail_pos = (
                    tail_pos.0 + ((head_pos.0 - tail_pos.0) as i32).signum(),
                    tail_pos.1 + ((head_pos.1 - tail_pos.1) as i32).signum(),
                );

                visited.insert(tail_pos);
            }
        }
    }

    Some(visited.len().try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let actions = input
        .lines()
        .map(|l| {
            let mut split = l.split(' ');

            (
                split.next().unwrap().chars().next().unwrap(),
                split.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut head_pos = (0, 0);
    let mut positions = (0..9).map(|_| (0, 0)).collect::<Vec<_>>();

    let mut visited = HashSet::new();
    visited.insert((0, 0));

    let distance_to_big = |lhs: (i32, i32), rhs: (i32, i32)| {
        ((lhs.0 - rhs.0) as i32).abs() >= 2 || ((lhs.1 - rhs.1) as i32).abs() >= 2
    };

    for action in actions {
        for _ in 0..action.1 {
            head_pos = match action.0 {
                'U' => (head_pos.0 + 1, head_pos.1),
                'D' => (head_pos.0 - 1, head_pos.1),
                'L' => (head_pos.0, head_pos.1 - 1),
                'R' => (head_pos.0, head_pos.1 + 1),
                _ => panic!("Invalid variant"),
            };

            let mut last = head_pos;

            for position in positions.iter_mut() {
                if distance_to_big(last, position.clone()) {
                    *position = (
                        position.0 + ((last.0 - position.0) as i32).signum(),
                        position.1 + ((last.1 - position.1) as i32).signum(),
                    );
                }

                last = position.clone();
            }

            visited.insert(positions.last().unwrap().clone());
        }
    }

    Some(visited.len().try_into().unwrap())
}

fn main() {
    let input = &aoc::read_file("inputs", 9);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(88));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
