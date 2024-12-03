advent_of_code::solution!(3);

use regex::Regex;

fn sum_of_mults(input: &str) -> u32 {
    let mult_re: Regex = Regex::new("mul\\(([0-9]+),([0-9]+)\\)").unwrap();

    let match_groups = mult_re
        .captures_iter(input)
        .map(|captures| captures.extract());

    let mut total_operations_sum: u32 = 0;

    for (_, [arg1_str, arg2_str]) in match_groups {
        let arg1 = arg1_str.parse::<u32>().unwrap();
        let arg2 = arg2_str.parse::<u32>().unwrap();

        total_operations_sum += arg1 * arg2;
    }

    return total_operations_sum;
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(sum_of_mults(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
