advent_of_code::solution!(1);

fn input_to_sorted_pair_vec(input: &str) -> Vec<Vec<u32>> {
    let mut left_vec = Vec::new();
    let mut right_vec = Vec::new();

    for line in input.lines() {
        let value_vec = line
            .split_whitespace()
            .map(|value| value.parse::<u32>().expect("Couldn't parse string to int."))
            .collect::<Vec<u32>>();

        let [left, right] = value_vec.as_slice() else {
            panic!("Not exactly two values to unpack.")
        };

        left_vec.push(left.to_owned());
        right_vec.push(right.to_owned());
    }

    let mut pair_vec = vec![left_vec, right_vec];

    for vec in pair_vec.iter_mut() {
        vec.sort()
    }

    return pair_vec;
}

pub fn part_one(input: &str) -> Option<i32> {
    let pair_vec = input_to_sorted_pair_vec(input);

    let [left_vec, right_vec] = pair_vec.as_slice() else {
        panic!("Can't unpack values.")
    };

    let mut total_distance: i32 = 0;

    for pair in left_vec.iter().zip(right_vec.iter()) {
        let (left, right) = pair;
        let diff: i32 = *left as i32 - *right as i32;
        total_distance += diff.abs();
    }

    Some(total_distance)
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
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(31));
    }
}
