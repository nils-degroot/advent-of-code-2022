pub fn part_one(input: &str) -> Option<u32> {
    let score = input
        .lines()
        .map(|l| {
            (
                Strategy::from(l.chars().nth(2).unwrap()),
                Strategy::from(l.chars().nth(0).unwrap()),
            )
        })
        .fold(0, |acc, (l, r)| {
            let fig_score = match l {
                Strategy::Rock => 1,
                Strategy::Paper => 2,
                Strategy::Scissors => 3,
            };
            let vic_score = match l.fight(&r) {
                Victory::Win => 6,
                Victory::Draw => 3,
                Victory::Loss => 0,
            };

            acc + fig_score + vic_score
        });

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let score = input
        .lines()
        .map(|l| {
            (
                Victory::from(l.chars().nth(2).unwrap()),
                Strategy::from(l.chars().nth(0).unwrap()),
            )
        })
        .fold(0, |acc, (l, r)| {
            let fig_score = match r.inverse_fight(&l) {
                Strategy::Rock => 1,
                Strategy::Paper => 2,
                Strategy::Scissors => 3,
            };
            let vic_score = match l {
                Victory::Win => 6,
                Victory::Draw => 3,
                Victory::Loss => 0,
            };

            acc + fig_score + vic_score
        });

    Some(score)
}

#[derive(Debug)]
enum Victory {
    Win,
    Draw,
    Loss,
}

impl From<char> for Victory {
    fn from(c: char) -> Self {
        match c {
            'X' => Self::Loss,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!("Invalid variant"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Strategy {
    Rock,
    Paper,
    Scissors,
}

impl Strategy {
    fn fight(&self, other: &Self) -> Victory {
        match (self, other) {
            (Strategy::Rock, Strategy::Rock) => Victory::Draw,
            (Strategy::Paper, Strategy::Paper) => Victory::Draw,
            (Strategy::Scissors, Strategy::Scissors) => Victory::Draw,

            (Strategy::Rock, Strategy::Scissors) => Victory::Win,
            (Strategy::Paper, Strategy::Rock) => Victory::Win,
            (Strategy::Scissors, Strategy::Paper) => Victory::Win,

            (Strategy::Rock, Strategy::Paper) => Victory::Loss,
            (Strategy::Paper, Strategy::Scissors) => Victory::Loss,
            (Strategy::Scissors, Strategy::Rock) => Victory::Loss,
        }
    }

    fn inverse_fight(&self, desired: &Victory) -> Self {
        match desired {
            Victory::Win => match self {
                Self::Rock => Self::Paper,
                Self::Paper => Self::Scissors,
                Self::Scissors => Self::Rock,
            },
            Victory::Draw => *self,
            Victory::Loss => match self {
                Self::Rock => Self::Scissors,
                Self::Paper => Self::Rock,
                Self::Scissors => Self::Paper,
            },
        }
    }
}

impl From<char> for Strategy {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("Invalid variant"),
        }
    }
}

fn main() {
    let input = &aoc::read_file("inputs", 2);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
