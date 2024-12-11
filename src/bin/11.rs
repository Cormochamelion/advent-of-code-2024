advent_of_code::solution!(11);

use std::collections::VecDeque;

fn parse_input(input: &str) -> VecDeque<u64> {
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn blink(stones: &mut VecDeque<u64>) {
    let mut stone: u64;
    let mut n_digits = 0;
    let mut downshift: u64;
    let mut left_digits: u64;
    let mut right_digits: u64;

    let mut i = 0;

    while i < stones.len() {
        stone = stones.remove(i).unwrap();

        if stone > 0u64 {
            n_digits = stone.ilog10() + 1;
        }

        if stone == 0u64 {
            stones.insert(i, 1u64);

            i += 1;
        } else if n_digits.rem_euclid(2) == 0 {
            downshift = 10u64.pow(n_digits / 2);
            left_digits = stone / downshift;
            stones.insert(i, left_digits);
            i += 1;

            right_digits = stone - (left_digits * downshift);
            stones.insert(i, right_digits);
            i += 1;
        } else {
            stones.insert(i, stone * 2024);

            i += 1;
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut stones = parse_input(input);

    let n_iter = 25;

    for _ in 0..n_iter {
        blink(&mut stones);
    }

    Some(stones.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
