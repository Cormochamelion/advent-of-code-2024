advent_of_code::solution!(13);

use nalgebra::{DMatrix, DVector};
use regex::Regex;

type Button = ([u32; 2], u32);
type PrizePos = [u32; 2];
type ClawMachine = ([Button; 2], PrizePos);

fn prize_for_button_type(button_type: &str) -> u32 {
    match button_type {
        "A" => 3,
        "B" => 1,
        _ => {
            panic!("Don't know price for button type {}.", button_type);
        }
    }
}

fn parse_input(input: &str) -> Vec<ClawMachine> {
    let mut output = Vec::new();

    let button_re = Regex::new(r"^Button ([A-Z]): X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let prize_re = Regex::new(r"^Prize: X=([0-9]+), Y=([0-9]+)").unwrap();

    let mut buttons: Vec<Button> = Vec::new();
    let mut prize_pos: PrizePos;
    let mut button_pos: [u32; 2];

    for line in input.lines() {
        if let Some((_, [button_type, x, y])) = button_re.captures(line).map(|caps| caps.extract())
        {
            button_pos = [x.parse().unwrap(), y.parse().unwrap()];
            buttons.push((button_pos, prize_for_button_type(button_type)));

            continue;
        }

        if let Some((_, [x, y])) = prize_re.captures(line).map(|caps| caps.extract()) {
            prize_pos = [x.parse().unwrap(), y.parse().unwrap()];

            output.push((buttons.clone().try_into().unwrap(), prize_pos));
            buttons.clear();
        }
    }

    output
}

fn find_cheapest_inputs(machine: ClawMachine) -> u32 {
    let precision = 10;
    let (buttons, prize_pos) = machine;
    let button_dirs: [[f64; 2]; 2] = buttons
        .iter()
        .map(|(button, _)| [button[0] as f64, button[1] as f64])
        .collect::<Vec<[f64; 2]>>()
        .try_into()
        .unwrap();

    let button_costs: [&u32; 2] = buttons
        .iter()
        .map(|(_, cost)| cost)
        .collect::<Vec<&u32>>()
        .try_into()
        .unwrap();

    let a_mat: DMatrix<f64> = DMatrix::from_columns(&[
        DVector::from_column_slice(&button_dirs[0]),
        DVector::from_column_slice(&button_dirs[1]),
    ]);
    let b_vec: DVector<f64> =
        DVector::from_column_slice(&[prize_pos[0] as f64, prize_pos[1] as f64]);

    let a_lu = a_mat.full_piv_lu();

    let float_solution = match a_lu.solve(&b_vec) {
        Some(solution) => solution,
        None => return 0,
    };

    let sol_slice = float_solution.as_slice();

    if sol_slice
        .iter()
        .all(|val| (val - val.round()).abs() < 10f64.powi(-precision))
    {
        let total_cost = sol_slice
            .iter()
            .zip(button_costs)
            .map(|(count, cost)| (count.round() as u32) * cost)
            .sum();
        total_cost
    } else {
        0
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let machines = parse_input(input);
    let mut least_tokens: u32 = 0;

    for machine in machines {
        least_tokens += find_cheapest_inputs(machine);
    }
    Some(least_tokens)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
