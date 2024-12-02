advent_of_code::solution!(2);

fn compare_levels<F>(a: u32, b: u32, compare_fun: F) -> bool
where
    F: Fn(u32, u32) -> bool,
{
    compare_fun(a, b) & (a.abs_diff(b) <= 3)
}

fn is_safe(report: &str) -> bool {
    let mut report_iter = report
        .split_whitespace()
        .map(|value| value.parse::<u32>().expect("Couldn't parse string to int."));

    let mut previous = report_iter
        .next()
        .expect("Report does not contain enough levels.");

    let current = report_iter
        .next()
        .expect("Report does not contain enough levels.");

    let compare_fun: fn(u32, u32) -> bool = if previous > current {
        |a, b| a > b
    } else if previous < current {
        |a, b| a < b
    } else {
        |_, _| false
    };

    if !compare_levels(previous, current, compare_fun) {
        return false;
    }

    previous = current;

    for current in report_iter {
        if !compare_levels(previous, current, compare_fun) {
            return false;
        }

        previous = current;
    }

    return true;
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut n_safe: u32 = 0;

    for report_string in input.lines() {
        if is_safe(report_string) {
            n_safe += 1
        };
    }

    Some(n_safe)
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
