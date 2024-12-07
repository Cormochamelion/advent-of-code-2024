advent_of_code::solution!(7);

use itertools::{repeat_n, Itertools};

fn add(a: u64, b: u64) -> u64 {
    a.checked_add(b).unwrap()
}

fn mul(a: u64, b: u64) -> u64 {
    a.checked_mul(b).unwrap()
}

const OPERATORS: [fn(u64, u64) -> u64; 2] = [add, mul];

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    let mut output = Vec::new();

    for line in input.lines() {
        let (cal_result, vals_str) = line.split_once(": ").unwrap();
        let cal_values = vals_str.split_whitespace();

        output.push((
            cal_result.parse().unwrap(),
            cal_values.map(|val_str| val_str.parse().unwrap()).collect(),
        ))
    }

    return output;
}

fn calibration_correct(calibration: &(u64, Vec<u64>), operators: &[fn(u64, u64) -> u64]) -> bool {
    let (cal_result, cal_values) = calibration;
    let n_operators = cal_values.len() - 1;

    for operator_seq in repeat_n(operators.iter(), n_operators).multi_cartesian_product() {
        let mut values_iter = cal_values.iter();
        let first_value = values_iter.next().unwrap();

        let result = values_iter
            .zip(operator_seq)
            .fold(first_value.clone(), |acc, (val, op)| op(acc, *val));

        if result == *cal_result {
            return true;
        }
    }

    return false;
}

pub fn part_one(input: &str) -> Option<u64> {
    let calibrations = parse_input(input);

    let mut sum_tot_cals = 0;

    for calibration in calibrations {
        if calibration_correct(&calibration, &OPERATORS) {
            sum_tot_cals += calibration.0;
        }
    }

    Some(sum_tot_cals)
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
