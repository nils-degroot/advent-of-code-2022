use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let mut reg_x = 1;
    let mut cycle = 0;
    let mut check_cycles = vec![220, 180, 140, 100, 60, 20];
    let mut signal = 0;

    for line in input.lines() {
        let mut parts = line.split(' ');
        let cmd = parts.next().unwrap();

        match cmd {
            "noop" => {
                cycle += 1;
            }
            "addx" => {
                if check_cycles.last().unwrap() <= &(cycle + 2) {
                    let multiplier = check_cycles.pop().unwrap();
                    signal += reg_x * multiplier;

                    if check_cycles.is_empty() {
                        break;
                    }
                }

                reg_x += parts.next().unwrap().parse::<i32>().unwrap();
                cycle += 2;
            }
            _ => panic!("Invalid variant"),
        }
    }

    Some(signal.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<String> {
    let mut reg_x = 1;
    let mut cycle = 0;

    let mut drawn = (0..6).map(|_| vec![]).collect::<Vec<Vec<char>>>();
    let mut draw = |inc: i32| {
        let row = drawn
            .get_mut((cycle.clone() as f32 / 40f32).floor() as usize)
            .unwrap();

        row.push(
            if ((cycle % 40 - 1)..=(cycle % 40 + 1)).contains(&reg_x.clone()) {
                '#'
            } else {
                ' '
            },
        );

        cycle += 1;
        reg_x += inc;
    };

    for line in input.lines() {
        let mut parts = line.split(' ');
        let cmd = parts.next().unwrap();

        match cmd {
            "noop" => {
                draw(0);
            }
            "addx" => {
                draw(0);
                draw(parts.next().unwrap().parse().unwrap());
            }
            _ => panic!("Invalid variant"),
        }
    }

    Some(
        drawn
            .iter()
            .map(|l| l.iter().collect::<String>())
            .join("\n"),
    )
}

fn main() {
    let input = &aoc::read_file("inputs", 10);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 10);
        assert_eq!(part_two(&input), Some("##  ##  ##  ##  ##  ##  ##  ##  ##  ##  \n###   ###   ###   ###   ###   ###   ### \n####    ####    ####    ####    ####    \n#####     #####     #####     #####     \n######      ######      ######      ####\n#######       #######       #######     ".to_string()));
    }
}
