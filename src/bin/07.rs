pub fn part_one(input: &str) -> Option<usize> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut active = vec![];
    let mut closed = vec![];

    for line in lines {
        match line {
            line if line.starts_with("$ cd") => {
                if &line[5..] == ".." {
                    closed.push(active.pop().unwrap())
                } else {
                    active.push(0)
                }
            }
            line if !line.starts_with("$ ls") && !line.starts_with("dir") => {
                let size = line[0..(line.find(|c: char| c.is_whitespace()).unwrap())]
                    .parse::<usize>()
                    .unwrap();

                active = active.into_iter().map(|n| n + size).collect::<Vec<_>>();
            }
            _ => (),
        }
    }

    closed.extend(active);

    Some(closed.into_iter().filter(|n| n <= &100000).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut active = vec![];
    let mut closed = vec![];

    for line in lines {
        match line {
            line if line.starts_with("$ cd") => {
                if &line[5..] == ".." {
                    closed.push(active.pop().unwrap())
                } else {
                    active.push(0)
                }
            }
            line if !line.starts_with("$ ls") && !line.starts_with("dir") => {
                let size = line[0..(line.find(|c: char| c.is_whitespace()).unwrap())]
                    .parse::<usize>()
                    .unwrap();

                active = active.into_iter().map(|n| n + size).collect::<Vec<_>>();
            }
            _ => (),
        }
    }

    closed.extend(active);

    let tmp = closed.clone();
    let required = 30000000 - (70000000 - tmp.iter().max().unwrap());

    Some(
        closed
            .into_iter()
            .filter(|size| *size >= required)
            .min()
            .unwrap(),
    )
}

fn main() {
    let input = &aoc::read_file("inputs", 7);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
