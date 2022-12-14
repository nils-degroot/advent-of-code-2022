use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
enum Space {
    Sand,
    Rock,
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut source_matrix = input
        .lines()
        .flat_map(|l| {
            let mut parts = l.split(" -> ").map(|p| {
                let mut parts = p.split(",").map(|n| n.parse::<u32>().unwrap());
                (parts.next().unwrap(), parts.next().unwrap())
            });

            let mut last = (0, 0);
            let mut acc = HashSet::new();
            while let Some(part) = parts.next() {
                if last == (0, 0) {
                    last = part.clone();
                }

                let coords = if last.0 == part.0 {
                    (part.1.min(last.1)..=part.1.max(last.1))
                        .map(|y| (part.0, y))
                        .collect::<Vec<_>>()
                } else if last.1 == last.1 {
                    (part.0.min(last.0)..=part.0.max(last.0))
                        .map(|x| (x, part.1))
                        .collect::<Vec<_>>()
                } else {
                    vec![part]
                };

                last = part;

                for coord in coords {
                    acc.insert(coord);
                }
            }

            acc
        })
        .map(|c| (c, Space::Rock))
        .collect::<HashMap<(u32, u32), Space>>();

    loop {
        let mut pos = (500, 0);

        loop {
            if source_matrix.contains_key(&(pos.0, pos.1 + 1)) {
                if !source_matrix.contains_key(&(pos.0 - 1, pos.1 + 1)) {
                    pos = (pos.0 - 1, pos.1);
                    continue;
                }

                if !source_matrix.contains_key(&(pos.0 + 1, pos.1 + 1)) {
                    pos = (pos.0 + 1, pos.1);
                    continue;
                }

                source_matrix.insert(pos, Space::Sand);
                break;
            } else {
                if pos.1 > 1000000 {
                    return Some(
                        source_matrix
                            .values()
                            .filter(|v| matches!(v, Space::Sand))
                            .count(),
                    );
                }

                pos = (pos.0, pos.1 + 1);
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut source_matrix = input
        .lines()
        .flat_map(|l| {
            let mut parts = l.split(" -> ").map(|p| {
                let mut parts = p.split(",").map(|n| n.parse::<u32>().unwrap());
                (parts.next().unwrap(), parts.next().unwrap())
            });

            let mut last = (0, 0);
            let mut acc = HashSet::new();
            while let Some(part) = parts.next() {
                if last == (0, 0) {
                    last = part.clone();
                }

                let coords = if last.0 == part.0 {
                    (part.1.min(last.1)..=part.1.max(last.1))
                        .map(|y| (part.0, y))
                        .collect::<Vec<_>>()
                } else if last.1 == last.1 {
                    (part.0.min(last.0)..=part.0.max(last.0))
                        .map(|x| (x, part.1))
                        .collect::<Vec<_>>()
                } else {
                    vec![part]
                };

                last = part;

                for coord in coords {
                    acc.insert(coord);
                }
            }

            acc
        })
        .map(|c| (c, Space::Rock))
        .collect::<HashMap<(u32, u32), Space>>();

    let src_clone = source_matrix.clone();
    let (_, lowest_y) = src_clone
        .keys()
        .max_by(|(_, l_y), (_, r_y)| l_y.cmp(r_y))
        .unwrap();

    for x in 0..1000 {
        source_matrix.insert((x, lowest_y + 2), Space::Rock);
    }

    loop {
        let mut pos = (500, 0);

        if source_matrix.contains_key(&pos) {
            break Some(
                source_matrix
                    .values()
                    .filter(|v| matches!(v, Space::Sand))
                    .count(),
            );
        }

        loop {
            if source_matrix.contains_key(&(pos.0, pos.1 + 1)) {
                if !source_matrix.contains_key(&(pos.0 - 1, pos.1 + 1)) {
                    pos = (pos.0 - 1, pos.1);
                    continue;
                }

                if !source_matrix.contains_key(&(pos.0 + 1, pos.1 + 1)) {
                    pos = (pos.0 + 1, pos.1);
                    continue;
                }

                source_matrix.insert(pos, Space::Sand);
                break;
            } else {
                pos = (pos.0, pos.1 + 1);
            }
        }
    }
}

fn main() {
    let input = &aoc::read_file("inputs", 14);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
