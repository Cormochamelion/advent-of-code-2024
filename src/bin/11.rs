advent_of_code::solution!(11);

use std::collections::{HashMap, VecDeque};

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

fn parse_input_compressed(input: &str) -> HashMap<u64, u64> {
    let mut input_map = HashMap::new();

    for stone in input.split_whitespace().map(|x| x.parse::<u64>().unwrap()) {
        match input_map.get_mut(&stone) {
            Some(entry) => {
                *entry += 1;
            }
            None => {
                input_map.insert(stone, 1);
            }
        }
    }

    input_map
}

fn transmute_stone(stone: &u64) -> Vec<u64> {
    let mut n_digits = 0;

    if stone > &0u64 {
        n_digits = stone.ilog10() + 1;
    }

    if stone == &0u64 {
        return vec![1];
    } else if n_digits.rem_euclid(2) == 0 {
        let downshift = 10u64.pow(n_digits / 2);
        let left_digits = stone / downshift;
        let right_digits = stone - (left_digits * downshift);

        return vec![left_digits, right_digits];
    } else {
        return vec![stone * 2024];
    }
}

fn blink_compressed(stones: &mut HashMap<u64, u64>) {
    let mut next_stones = HashMap::new();
    let mut stone_products: Vec<u64>;

    for (stone_val, count) in stones.iter() {
        stone_products = transmute_stone(stone_val);

        for product in stone_products.drain(..) {
            match next_stones.get_mut(&product) {
                Some(entry) => {
                    *entry += count;
                }
                None => {
                    next_stones.insert(product, *count);
                }
            }
        }
    }

    *stones = next_stones;
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut stones = parse_input_compressed(input);

    let n_iter = 75;

    for _ in 0..n_iter {
        blink_compressed(&mut stones);
    }

    Some(stones.iter().map(|(_, count)| count).sum())
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
        assert_eq!(result, Some(65601038650482));
    }
}
