use std::collections::HashSet;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    let chars = input
        .lines()
        .flat_map(|l| {
            let comp_1 = &l[..(l.len() / 2)];
            let comp_2 = &l[(l.len() / 2)..];

            comp_1
                .chars()
                .filter(|c| comp_2.contains(*c))
                .collect::<HashSet<_>>()
        })
        .map(|c| c)
        .collect::<Vec<_>>();

    let mut range = ('a'..='z').collect::<Vec<_>>();
    range.append(&mut ('A'..='Z').collect::<Vec<_>>());

    let result = chars
        .into_iter()
        .map(|c| range.iter().position(|i| &c == i).unwrap() + 1)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let chunks = input.lines().chunks(3);

    let chars: &Vec<char> = &chunks
        .into_iter()
        .map(|l| {
            let part = l.into_iter().collect::<Vec<_>>();

            let line_1 = part.get(1).unwrap();
            let line_2 = part.get(2).unwrap();

            part.get(0)
                .unwrap()
                .chars()
                .find(|c| line_1.contains(*c) && line_2.contains(*c))
                .unwrap()
        })
        .map(|c| c)
        .collect::<Vec<_>>();

    let mut range = ('a'..='z').collect::<Vec<_>>();
    range.append(&mut ('A'..='Z').collect::<Vec<_>>());

    let result = chars
        .into_iter()
        .map(|c| range.iter().position(|i| c == i).unwrap() + 1)
        .sum();

    Some(result)
}

fn main() {
    let input = &aoc::read_file("inputs", 3);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
