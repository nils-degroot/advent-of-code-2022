pub fn part_one(input: &str) -> Option<u32> {
    let mut top = 0;
    let mut acc = 0;

    for line in input.lines() {
        if line.is_empty() {
            if acc > top {
                top = acc;
            }

            acc = 0;
        } else {
            acc += line.parse::<u32>().unwrap();
        }
    }

    Some(top)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total = vec![];
    let mut acc = 0;

    for line in input.lines() {
        if line.is_empty() {
            total.push(acc);
            acc = 0;
        } else {
            acc += line.parse::<u32>().unwrap();
        }
    }
    total.push(acc);

    total.sort();
    total.reverse();

    Some(total.into_iter().take(3).sum())
}

fn main() {
    let input = &aoc::read_file("inputs", 1);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
