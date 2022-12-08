use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let heigth = grid.len();
    let width = grid.get(0).unwrap().len();
    let mut visible = 0;

    for x in 0..grid.len() {
        let row = grid.get(x).unwrap();

        for y in 0..grid.first().unwrap().len() {
            let current = row.get(y).unwrap();

            if x == 0 || y == 0 || x == heigth - 1 || y == width - 1 {
                visible += 1;
                continue;
            }

            let r_to_l = ((y + 1)..width).map(|i| row.get(i).unwrap()).max().unwrap() < current;
            let l_to_r = (0..y).map(|i| row.get(i).unwrap()).max().unwrap() < current;
            let t_to_b = (0..x)
                .map(|i| grid.get(i).unwrap().get(y).unwrap())
                .max()
                .unwrap()
                < current;
            let b_to_t = ((x + 1)..heigth)
                .map(|i| grid.get(i).unwrap().get(y).unwrap())
                .max()
                .unwrap()
                < current;

            visible += if r_to_l || l_to_r || t_to_b || b_to_t {
                1
            } else {
                0
            };
        }
    }

    Some(visible)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let heigth = grid.len();
    let width = grid.get(0).unwrap().len();
    let mut highest_score = 0;

    for x in 0..grid.len() {
        let row = grid.get(x).unwrap();

        for y in 0..grid.first().unwrap().len() {
            if x == 0 || y == 0 || x == heigth - 1 || y == width - 1 {
                continue;
            }

            let current = row.get(y).unwrap();
            let cmp = |acc: i32, n: &u32| {
                if current > n {
                    Continue(acc + 1)
                } else {
                    Done(acc + 1)
                }
            };

            let r_to_l = ((y + 1)..width)
                .map(|i| row.get(i).unwrap())
                .fold_while(0, cmp)
                .into_inner();

            let l_to_r = (0..y)
                .rev()
                .map(|i| row.get(i).unwrap())
                .fold_while(0, cmp)
                .into_inner();

            let t_to_b = (0..x)
                .rev()
                .map(|i| grid.get(i).unwrap().get(y).unwrap())
                .fold_while(0, cmp)
                .into_inner();

            let b_to_t = ((x + 1)..heigth)
                .map(|i| grid.get(i).unwrap().get(y).unwrap())
                .fold_while(0, cmp)
                .into_inner();

            if r_to_l * l_to_r * t_to_b * b_to_t > highest_score {
                highest_score = r_to_l * l_to_r * t_to_b * b_to_t;
            }
        }
    }

    Some(highest_score.try_into().unwrap())
}

fn main() {
    let input = &aoc::read_file("inputs", 8);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
