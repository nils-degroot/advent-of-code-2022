use itertools::Itertools;
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let i = input
        .chars()
        .tuple_windows::<(char, char, char, char)>()
        .position(|w| {
            let mut set = HashSet::new();
            set.insert(w.0);
            set.insert(w.1);
            set.insert(w.2);
            set.insert(w.3);

            set.len() == 4
        })
        .unwrap();

    Some((i + 4).try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let chars = input.chars().collect::<Vec<_>>();

    let mut i = 0;
    loop {
        if chars[i..(i + 14)].iter().collect::<HashSet<_>>().len() == 14 {
            break Some((i + 14).try_into().unwrap());
        }

        i += 1;
    }
}

fn main() {
    let input = &aoc::read_file("inputs", 6);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
