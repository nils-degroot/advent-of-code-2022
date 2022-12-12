use std::collections::{BTreeMap, BinaryHeap};

pub fn part_one(input: &str) -> Option<usize> {
    let start_pos = input
        .lines()
        .enumerate()
        .find_map(|(x, l)| l.chars().position(|c| c == 'S').map(|y| (x, y)))
        .unwrap();

    let end_pos = input
        .lines()
        .enumerate()
        .find_map(|(x, l)| l.chars().position(|c| c == 'E').map(|y| (x, y)))
        .unwrap();

    let input = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    if c == 'S' {
                        0
                    } else if c == 'E' {
                        27
                    } else {
                        c as u8 - b'a' + 1
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let neighbors = |(x, y): (usize, usize)| {
        let mut acc = vec![];
        let current = input[x][y];

        let mut check = |x: usize, y: usize| {
            if current + 1 >= input[x][y] {
                acc.push((x, y));
            }
        };

        if x > 0 {
            check(x - 1, y);
        }
        if y > 0 {
            check(x, y - 1);
        }
        if x < input.len() - 1 {
            check(x + 1, y);
        }
        if y < input.first().unwrap().len() - 1 {
            check(x, y + 1);
        }

        acc
    };

    let mut costs: BTreeMap<(usize, usize), usize> = BTreeMap::new();
    costs.insert(start_pos, 0);
    let mut heap = BinaryHeap::new();
    heap.push((start_pos, 0));

    while let Some((pos, old_cost)) = heap.pop() {
        if pos == end_pos {
            continue;
        }

        for (x, y) in neighbors(pos) {
            let new_cost = old_cost + 1;

            if new_cost < *costs.get(&(x, y)).unwrap_or(&usize::MAX) {
                heap.push(((x, y), new_cost));
                costs.insert((x, y), new_cost);
            }
        }
    }

    Some(*costs.get(&end_pos).unwrap())
}

pub fn part_two(input: &str) -> Option<usize> {
    let end_pos = input
        .lines()
        .enumerate()
        .find_map(|(x, l)| l.chars().position(|c| c == 'E').map(|y| (x, y)))
        .unwrap();

    let input = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    if c == 'S' {
                        1
                    } else if c == 'E' {
                        27
                    } else {
                        c as u8 - b'a' + 1
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let neighbors = |(x, y): (usize, usize)| {
        let mut acc = vec![];
        let current = input[x][y];

        let mut check = |x: usize, y: usize| {
            if current + 1 >= input[x][y] {
                acc.push((x, y));
            }
        };

        if x > 0 {
            check(x - 1, y);
        }
        if y > 0 {
            check(x, y - 1);
        }
        if x < input.len() - 1 {
            check(x + 1, y);
        }
        if y < input.first().unwrap().len() - 1 {
            check(x, y + 1);
        }

        acc
    };

    let mut lowest = usize::MAX;
    let starting_points = input
        .clone()
        .into_iter()
        .enumerate()
        .flat_map(|(x, r)| {
            r.into_iter()
                .enumerate()
                .filter(|(_, c)| c == &1)
                .map(move |(y, _)| (x, y))
        })
        .collect::<Vec<_>>();

    for start_pos in starting_points {
        let mut costs: BTreeMap<(usize, usize), usize> = BTreeMap::new();
        costs.insert(start_pos, 0);
        let mut heap = BinaryHeap::new();
        heap.push((start_pos, 0));

        while let Some((pos, old_cost)) = heap.pop() {
            if pos == end_pos {
                continue;
            }

            for (x, y) in neighbors(pos) {
                let new_cost = old_cost + 1;

                if new_cost < *costs.get(&(x, y)).unwrap_or(&usize::MAX) {
                    heap.push(((x, y), new_cost));
                    costs.insert((x, y), new_cost);
                }
            }
        }

        if *costs.get(&end_pos).unwrap_or(&usize::MAX) < lowest {
            lowest = *costs.get(&end_pos).unwrap();
        }
    }

    Some(lowest)
}

fn main() {
    let input = &aoc::read_file("inputs", 12);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
