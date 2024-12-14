advent_of_code::solution!(14);

use itertools::izip;
use regex::Regex;

fn parse_input(input: &str) -> (Vec<[usize; 2]>, Vec<[i32; 2]>, [usize; 2]) {
    let robot_re =
        Regex::new(r"^p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)$").unwrap();

    // I have appended the test dimensions to the test output. If that line is
    // not found, the function will output the solution dimensions.
    let dim_re = Regex::new(r"^\(([0-9]+), ([0-9]+)\)$").unwrap();

    let mut init_pos = Vec::new();
    let mut velos = Vec::new();
    let mut dims = [103, 101];

    for line in input.lines() {
        if let Some((_, [pos_x, pos_y, vel_x, vel_y])) =
            robot_re.captures(line).map(|caps| caps.extract())
        {
            init_pos.push(
                [pos_y, pos_x]
                    .iter()
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<usize>>()
                    .try_into()
                    .unwrap(),
            );
            velos.push(
                [vel_y, vel_x]
                    .iter()
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<i32>>()
                    .try_into()
                    .unwrap(),
            );
        } else if let Some((_, [dim_x, dim_y])) =
            dim_re.captures(line).map(|caps| caps.extract())
        {
            dims = [dim_y, dim_x]
                .iter()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<usize>>()
                .try_into()
                .unwrap();
        }
    }

    (init_pos, velos, dims)
}

fn positions_after_n_seconds(
    init_pos: Vec<[usize; 2]>,
    velocities: Vec<[i32; 2]>,
    dims: [usize; 2],
    n_seconds: u32,
) -> Vec<[usize; 2]> {
    let mut new_pos = Vec::new();

    for (pos, velo) in init_pos.iter().zip(velocities) {
        let estim_pos = izip!(pos, velo, dims)
            .map(|(p, v, d)| {
                let travel_dist = n_seconds as i32 * v;
                let pos_after_travel = *p as i32 + travel_dist;

                (pos_after_travel.rem_euclid(d as i32)) as usize
            })
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap();

        new_pos.push(estim_pos);
    }

    new_pos
}

fn get_quadrant(pos: [usize; 2], quad_lines: [usize; 2]) -> Option<usize> {
    let quad_pattern: [bool; 2] = match pos
        .iter()
        .zip(quad_lines)
        // Filter a dim on a line.
        .filter(|(p, q)| *p < q || q < *p)
        .map(|(p, q)| *p < q)
        .collect::<Vec<bool>>()
        .try_into()
    {
        Ok(patt) => patt,
        Err(_) => return None,
    };

    match quad_pattern {
        [true, true] => Some(0),
        [true, false] => Some(1),
        [false, true] => Some(2),
        [false, false] => Some(3),
    }
}

fn get_safety_factor(pred_pos: Vec<[usize; 2]>, dims: [usize; 2]) -> u32 {
    let mut quad_counts = [0, 0, 0, 0];
    let quad_lines: [usize; 2] = dims
        .iter()
        // Avoid off-by one error; the lines need to correspond to indices.
        .map(|d| d.div_ceil(2) - 1)
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap();

    let mut quadrant: usize;

    for pos in pred_pos {
        quadrant = match get_quadrant(pos, quad_lines) {
            Some(quad) => quad,
            None => continue,
        };
        quad_counts[quadrant] += 1;
    }

    quad_counts.iter().product()
}

pub fn part_one(input: &str) -> Option<u32> {
    let n_seconds = 100;
    let (initial_positions, velocities, dims) = parse_input(input);
    let predicted_pos = positions_after_n_seconds(
        initial_positions,
        velocities,
        dims,
        n_seconds,
    );

    let safety_factor = get_safety_factor(predicted_pos, dims);

    Some(safety_factor)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
