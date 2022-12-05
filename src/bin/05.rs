pub fn part_one(input: &str) -> Option<String> {
    let input_end = input.lines().position(|l| l.is_empty()).unwrap();
    let width = (input.lines().next().unwrap().len() as f32 / 4.0).ceil() as usize;

    let mut start = input.lines().take(input_end - 1).collect::<Vec<_>>();
    start.reverse();

    let mut stacks = (0..width)
        .map(|i| {
            start
                .iter()
                .map(|l| l.chars().nth(i * 4 + 1).unwrap())
                .filter(|c| !c.is_whitespace())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let instructions = input
        .lines()
        .skip(input_end + 1)
        .map(|l| {
            let part = l.split(' ').collect::<Vec<_>>();

            (
                part.get(1).unwrap().parse::<usize>().unwrap(),
                part.get(3).unwrap().parse::<usize>().unwrap(),
                part.get(5).unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    for instruction in &instructions {
        for _ in 0..(instruction.0) {
            let popped = {
                let from = stacks.get_mut(instruction.1 - 1).unwrap();
                from.pop().unwrap()
            };

            stacks.get_mut(instruction.2 - 1).unwrap().push(popped);
        }
    }

    let result = stacks.iter().map(|s| s.last().unwrap()).collect::<String>();
    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let input_end = input.lines().position(|l| l.is_empty()).unwrap();
    let width = (input.lines().next().unwrap().len() as f32 / 4.0).ceil() as usize;

    let mut start = input.lines().take(input_end - 1).collect::<Vec<_>>();
    start.reverse();

    let mut stacks = (0..width)
        .map(|i| {
            start
                .iter()
                .map(|l| l.chars().nth(i * 4 + 1).unwrap())
                .filter(|c| !c.is_whitespace())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let instructions = input
        .lines()
        .skip(input_end + 1)
        .map(|l| {
            let part = l.split(' ').collect::<Vec<_>>();

            (
                part.get(1).unwrap().parse::<usize>().unwrap(),
                part.get(3).unwrap().parse::<usize>().unwrap(),
                part.get(5).unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    for instruction in &instructions {
        let mut popped = (0..(instruction.0))
            .map(|_| {
                let from = stacks.get_mut(instruction.1 - 1).unwrap();
                from.pop().unwrap()
            })
            .collect::<Vec<_>>();
        popped.reverse();

        stacks
            .get_mut(instruction.2 - 1)
            .unwrap()
            .append(&mut popped);
    }

    let result = stacks.iter().map(|s| s.last().unwrap()).collect::<String>();
    Some(result)
}

fn main() {
    let input = &aoc::read_file("inputs", 5);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
