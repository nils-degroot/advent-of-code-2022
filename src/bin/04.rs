pub fn part_one(input: &str) -> Option<usize> {
    let pairs = input
        .lines()
        .map(|l| {
            let parts = l.split(',').collect::<Vec<_>>();

            let ranges = parts
                .into_iter()
                .map(|p| {
                    let part = p
                        .split('-')
                        .map(|n| n.parse().unwrap())
                        .collect::<Vec<usize>>();

                    (*part.get(0).unwrap()..=*part.get(1).unwrap()).collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            (
                ranges.get(0).unwrap().clone(),
                ranges.get(1).unwrap().clone(),
            )
        })
        .collect::<Vec<_>>();

    let count = pairs
        .into_iter()
        .filter(|(l, r)| {
            if l.len() < r.len() {
                l.clone().into_iter().all(|c| r.contains(&c))
            } else {
                r.clone().into_iter().all(|c| l.contains(&c))
            }
        })
        .count();

    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let pairs = input
        .lines()
        .map(|l| {
            let parts = l.split(',').collect::<Vec<_>>();

            let ranges = parts
                .into_iter()
                .map(|p| {
                    let part = p
                        .split('-')
                        .map(|n| n.parse().unwrap())
                        .collect::<Vec<usize>>();

                    (*part.get(0).unwrap()..=*part.get(1).unwrap()).collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            (
                ranges.get(0).unwrap().clone(),
                ranges.get(1).unwrap().clone(),
            )
        })
        .collect::<Vec<_>>();

    let count = pairs
        .into_iter()
        .filter(|(l, r)| {
            if l.len() < r.len() {
                l.clone().into_iter().find(|c| r.contains(&c)).is_some()
            } else {
                r.clone().into_iter().find(|c| l.contains(&c)).is_some()
            }
        })
        .count();

    Some(count)
}

fn main() {
    let input = &aoc::read_file("inputs", 4);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
