use std::collections::HashSet;

use itertools::Itertools;

type Point = (i32, i32);

fn manhattan_distance(p1: &Point, p2: &Point) -> u32 {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = input
        .lines()
        .map(|l| {
            let sensor_x = &l[(l.find("x").unwrap() + 2)..l.find(",").unwrap()];
            let sensor_y = &l[(l.find("y").unwrap() + 2)..l.find(":").unwrap()];

            let beacon = &l[l.find(":").unwrap()..];
            let beacon_x = &beacon[(beacon.find("x").unwrap() + 2)..beacon.find(",").unwrap()];
            let beacon_y = &beacon[(beacon.find("y").unwrap() + 2)..];

            (
                (
                    sensor_x.parse::<i32>().unwrap(),
                    sensor_y.parse::<i32>().unwrap(),
                ),
                (
                    beacon_x.parse::<i32>().unwrap(),
                    beacon_y.parse::<i32>().unwrap(),
                ),
            )
        })
        .map(|(s, b)| (s, b, manhattan_distance(&s, &b) as i32))
        .collect::<Vec<_>>();

    let y = if cfg!(test) { 10 } else { 2000000 };

    let min_x = input.iter().map(|((x, _), _, d)| x - d).min().unwrap();
    let max_x = input.iter().map(|((x, _), _, d)| x + d).max().unwrap();
    let beacons = input.iter().map(|(_, b, _)| b).collect::<HashSet<_>>();

    let safe_positions = (min_x..=max_x)
        .filter(|x| {
            let pos = (*x, y);

            input.iter().any(|(s, _, d)| {
                manhattan_distance(s, &pos) as i32 <= *d && !beacons.contains(&pos)
            })
        })
        .collect::<Vec<_>>();

    Some(safe_positions.len().try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = input
        .lines()
        .map(|l| {
            let sensor_x = &l[(l.find("x").unwrap() + 2)..l.find(",").unwrap()];
            let sensor_y = &l[(l.find("y").unwrap() + 2)..l.find(":").unwrap()];

            let beacon = &l[l.find(":").unwrap()..];
            let beacon_x = &beacon[(beacon.find("x").unwrap() + 2)..beacon.find(",").unwrap()];
            let beacon_y = &beacon[(beacon.find("y").unwrap() + 2)..];

            (
                (
                    sensor_x.parse::<i32>().unwrap(),
                    sensor_y.parse::<i32>().unwrap(),
                ),
                (
                    beacon_x.parse::<i32>().unwrap(),
                    beacon_y.parse::<i32>().unwrap(),
                ),
            )
        })
        .map(|(s, b)| (s, b, manhattan_distance(&s, &b) as i32))
        .collect::<Vec<_>>();

    let max = if cfg!(test) { 20 } else { 4_000_000 };

    for y in 0..=max {
        let acc = input
            .iter()
            .filter_map(|(s, _, dist)| {
                let dist_y = s.1.abs_diff(y) as i32;
                let dist_x = dist - dist_y;

                (dist > &dist_y).then(|| (s.0 - dist_x, s.0 + dist_x))
            })
            .sorted()
            .collect::<Vec<_>>();

        let mut low = acc[0];
        let last = acc[1..].into_iter().find_map(|row| {
            if row.0 > low.1 {
                Some(row)
            } else {
                low = (low.0, row.1.max(low.1));
                None
            }
        });

        if last.is_some() {
            return Some(4_000_000 * (low.1 + 1) as usize + y as usize);
        }
    }

    panic!()
}

fn main() {
    let input = &aoc::read_file("inputs", 15);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(56000011));
    }
}
