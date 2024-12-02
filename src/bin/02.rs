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
        .map(|value| value.parse::<u32>().expect("Couldn't parse string to int."))
        .peekable();

    let mut previous = report_iter
        .next()
        .expect("Report does not contain enough levels.");

    let current_ref = report_iter.peek().unwrap();

    let compare_fun: fn(u32, u32) -> bool = if previous > *current_ref {
        |a, b| a > b
    } else if previous < *current_ref {
        |a, b| a < b
    } else {
        |_, _| false
    };

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

fn vec_without(vec: &Vec<u32>, i: usize) -> Vec<u32> {
    let mut new_vec = vec.clone();
    _ = new_vec.remove(i);
    return new_vec;
}

fn is_safe_dampened(mut report_vec: Vec<u32>, tol: u32) -> bool {
    let mut previous = report_vec[0];
    let next = report_vec[1];

    let compare_fun: fn(u32, u32) -> bool = if previous > next {
        |a, b| a > b
    } else if previous < next {
        |a, b| a < b
    } else {
        |_, _| false
    };

    let mut report_enum_iter = report_vec.iter().enumerate();

    // We already took `previous`, loop needs to start later.
    _ = report_enum_iter.next();

    for (i, &current) in report_enum_iter {
        if !compare_levels(previous, current, compare_fun) {
            if tol < 1 {
                return false;
            }

            let new_tol = tol - 1;

            let safe_without_current = is_safe_dampened(vec_without(&report_vec, i), new_tol);
            let safe_without_previous = is_safe_dampened(vec_without(&report_vec, i - 1), new_tol);
            let safe_without_first = is_safe_dampened(vec_without(&report_vec, 0), new_tol);

            return safe_without_current | safe_without_previous | safe_without_first;
        }

        previous = current;
    }

    return true;
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut n_safe: u32 = 0;

    for report_string in input.lines() {
        let report_vec = report_string
            .split_whitespace()
            .map(|value| value.parse::<u32>().expect("Couldn't parse string to int."))
            .collect::<Vec<u32>>();
        if is_safe_dampened(report_vec, 1) {
            n_safe += 1
        };
    }

    Some(n_safe)
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
        assert_eq!(result, Some(7));
    }
}
