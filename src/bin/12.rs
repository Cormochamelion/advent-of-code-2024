use std::collections::HashSet;

advent_of_code::solution!(12);

type Position = [usize; 2];
type Direction = [i32; 2];

const DIRECTIONS: [Direction; 4] = [[-1, 0], [0, -1], [0, 1], [1, 0]];

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut output = Vec::new();

    for line in input.lines() {
        output.push(line.chars().collect());
    }

    output
}

fn move_in_dir(pos: Position, dir: Direction) -> Position {
    pos.iter()
        .zip(dir)
        .map(|(pos_comp, dir_comp)| ((*pos_comp as i32) + dir_comp) as usize)
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap()
}

fn pos_is_valid(pos: Position, dims: [&usize; 2]) -> bool {
    pos.iter()
        .zip(dims)
        .all(|(pos_comp, dim_comp)| pos_comp < dim_comp)
}

fn explore_region(
    pos: Position,
    garden: &Vec<Vec<char>>,
    plant_type: &char,
    visited_plots: &mut HashSet<Position>,
    region: &mut Vec<Position>,
    perimeter_len: &mut usize,
) {
    let mut new_pos: Position;

    _ = visited_plots.insert(pos);

    region.push(pos.to_owned());

    for dir in DIRECTIONS {
        new_pos = move_in_dir(pos, dir);

        if !pos_is_valid(new_pos, [&garden.len(), &garden[pos[1]].len()]) {
            *perimeter_len += 1;
            continue;
        }

        if garden[new_pos[0]][new_pos[1]] != *plant_type {
            *perimeter_len += 1;
            continue;
        }

        if !visited_plots.contains(&new_pos) {
            explore_region(
                new_pos,
                garden,
                plant_type,
                visited_plots,
                region,
                perimeter_len,
            );
        }
    }
}

fn map_region<'a>(
    init_pos: Position,
    garden: &Vec<Vec<char>>,
    visited_plots: &'a mut HashSet<Position>,
) -> (Vec<Position>, usize) {
    let mut region: Vec<Position> = Vec::new();
    let mut perimeter_len: usize = 0;

    let plant_type = garden[init_pos[0]][init_pos[1]];

    explore_region(
        init_pos,
        garden,
        &plant_type,
        visited_plots,
        &mut region,
        &mut perimeter_len,
    );

    (region, perimeter_len)
}

pub fn part_one(input: &str) -> Option<u32> {
    // - Parse input into garden: Vec<Vec<char>>.
    // - Find regions:
    //   - Init regions: Vec<(Vec<(&usize, &usize)>, usize)> (Vector of
    //     positions, size of perimeter).
    //   - Init visited_plots: HashSet<Position>
    //   - For i in rows of garden, j each plot in row:
    //     - If (i, j) is not already in visited_plots:
    //       - ? Init region vec.
    //       - ? Init perimeter count.
    //       - Get the plant_type as garden[i][j].
    //       - Add (i, j) to visited plots.
    //       - Add &(i, j) to region_vec
    //       - For each direction (up, right, down, left):
    //          - If garden[direction] == plant_type:
    //            - Recurse in that direction.
    //          - Else: Increment perimeter count.
    //  - For each region in regions: Sum the product of the length of the
    //    perimeter with the number of plots.
    let garden = parse_input(input);
    let mut regions: Vec<(Vec<Position>, usize)> = Vec::new();
    let mut visited_plots: HashSet<Position> = HashSet::new();

    for i in 0..garden.len() {
        for j in 0..garden[i].len() {
            if visited_plots.contains(&[i, j]) {
                continue;
            } else {
                regions.push(map_region([i, j], &garden, &mut visited_plots));
            }
        }
    }

    Some(regions.iter().fold(0, |acc, (plots, perim_size)| {
        acc + ((plots.len() * (*perim_size)) as u32)
    }))
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
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
