use std::collections::HashSet;

advent_of_code::solution!(12);

type Position = [usize; 2];
type Direction = [i32; 2];
type Edge = (usize, usize, i32, i32);

const DIRECTIONS: [Direction; 4] = [[-1, 0], [0, -1], [0, 1], [1, 0]];

fn next_direction(dir: Direction) -> Direction {
    // Get the next straight direction counter-clockwise.
    match dir {
        // Up -> Left
        [-1, 0] => [0, -1],
        // Left -> Down
        [0, -1] => [1, 0],
        // Down -> Right
        [1, 0] => [0, 1],
        // Right -> Up
        [0, 1] => [-1, 0],
        _ => panic!(
            "Invalid direction: {:?}, needs to be one of {:?}.",
            dir, DIRECTIONS
        ),
    }
}

fn prev_diag(dir: Direction) -> Direction {
    // Get the previous diagonal to a straight direction counter-clockwise.
    match dir {
        // Up -> Right-Up
        [-1, 0] => [-1, 1],
        // Left -> Up-Left
        [0, -1] => [-1, -1],
        // Down -> Left-Down
        [1, 0] => [1, -1],
        // Right -> Down-Right
        [0, 1] => [1, 1],
        _ => panic!(
            "Invalid direction: {:?}, needs to be one of {:?}.",
            dir, DIRECTIONS
        ),
    }
}

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

fn map_region(
    init_pos: Position,
    garden: &Vec<Vec<char>>,
    visited_plots: &mut HashSet<Position>,
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

fn explore_region_lazy(
    pos: Position,
    garden: &Vec<Vec<char>>,
    plant_type: &char,
    visited_plots: &mut HashSet<Position>,
    region: &mut Vec<Position>,
    edges: &mut HashSet<Edge>,
) {
    let mut new_pos: Position;
    let mut new_pos_is_visited: bool;
    let mut potential_edge: Edge;

    _ = visited_plots.insert(pos);

    region.push(pos.to_owned());

    for dir in DIRECTIONS {
        new_pos = move_in_dir(pos, dir);
        potential_edge = (pos[0], pos[1], dir[0], dir[1]);

        if !pos_is_valid(new_pos, [&garden.len(), &garden[pos[1]].len()]) {
            edges.insert(potential_edge);
            continue;
        }

        if garden[new_pos[0]][new_pos[1]] != *plant_type {
            edges.insert(potential_edge);
            continue;
        }

        new_pos_is_visited = visited_plots.contains(&new_pos);

        if !new_pos_is_visited {
            explore_region_lazy(new_pos, garden, plant_type, visited_plots, region, edges);
        }
    }
}

fn get_next_edge(edge: Edge) -> Edge {
    let dir = next_direction([edge.2, edge.3]);
    let new_pos: Position = move_in_dir([edge.0, edge.1], dir);

    (new_pos[0], new_pos[1], edge.2, edge.3)
}

fn find_corner(start: Edge, edges: &mut HashSet<Edge>) -> Edge {
    let mut curr_edge = start;
    let mut next_edge: Edge = get_next_edge(start);

    while edges.contains(&next_edge) {
        curr_edge = next_edge;
        next_edge = get_next_edge(curr_edge);
    }

    curr_edge
}

fn find_cornering_edge(corner: Edge, edges: &mut HashSet<Edge>) -> Option<&Edge> {
    // Find possible edge at 90°.
    let right_angle_dir = next_direction([corner.2, corner.3]);
    let right_angle_edge = (corner.0, corner.1, right_angle_dir[0], right_angle_dir[1]);

    let maybe_right_angle = edges.get(&right_angle_edge);

    if maybe_right_angle.is_some() {
        return maybe_right_angle;
    }

    // Next edge would be at 270°.
    let oblique_angle_dir = next_direction(next_direction(right_angle_dir));
    let diag_dir = prev_diag(right_angle_dir);

    let oblique_anlge_pos = move_in_dir([corner.0, corner.1], diag_dir);
    let oblique_angle_edge = (
        oblique_anlge_pos[0],
        oblique_anlge_pos[1],
        oblique_angle_dir[0],
        oblique_angle_dir[1],
    );

    edges.get(&oblique_angle_edge)
}

fn find_corner_consuming(start: Edge, edges: &mut HashSet<Edge>) -> Edge {
    let mut curr_edge = start;
    let mut next_edge = get_next_edge(curr_edge);

    _ = edges.take(&curr_edge);

    while edges.take(&next_edge).is_some() {
        curr_edge = next_edge;
        next_edge = get_next_edge(curr_edge);
    }

    curr_edge
}

fn count_sides(edges: &mut HashSet<Edge>) -> usize {
    let mut n_sides: usize = 0;
    let mut next_corner: Edge;
    let mut arbitrary_edge: &Edge;

    while !edges.is_empty() {
        // Get some edge, which definetly belongs to a contiguous boundary.
        arbitrary_edge = edges.iter().next().unwrap();

        // To ensure all edges get visited, we can't start counting in the
        // middle of a side, so we start at some corner.
        next_corner = find_corner(*arbitrary_edge, edges);

        // Get the next side of the boundary. If there's none we're done with
        // this boundary and get the next.
        while let Some(cornering_edge) = find_cornering_edge(next_corner, edges) {
            // Find the next corner, removing every edge along the way.
            next_corner = find_corner_consuming(*cornering_edge, edges);
            n_sides += 1;
        }
    }

    n_sides
}

fn map_region_lazy(
    init_pos: Position,
    garden: &Vec<Vec<char>>,
    visited_plots: &mut HashSet<Position>,
) -> (Vec<Position>, usize) {
    let mut region: Vec<Position> = Vec::new();
    let mut edges: HashSet<Edge> = HashSet::new();

    let plant_type = garden[init_pos[0]][init_pos[1]];

    explore_region_lazy(
        init_pos,
        garden,
        &plant_type,
        visited_plots,
        &mut region,
        &mut edges,
    );

    (region, count_sides(&mut edges))
}

pub fn part_two(input: &str) -> Option<u32> {
    // - Parse as in part one.
    // - Find regions:
    //   - Init regions: Vec<(HashSet<(&usize, &usize)>, usize)> (Set of
    //     positions, number of sides).
    //   - For i in rows of garden, j each plot in row:
    //     - Init edges: HashSet<(usize, usize, int, int)> collecting each edge
    //       as pos with a direction.
    //     - Explore region as before.
    //       - Add egde to edges when previously we would have counted a
    //         perimeter.
    //     - Count sides by walking along each boundary, removing each edge
    //       and counting new corners as sides.
    let garden = parse_input(input);
    let mut regions: Vec<(Vec<Position>, usize)> = Vec::new();
    let mut visited_plots: HashSet<Position> = HashSet::new();

    for i in 0..garden.len() {
        for j in 0..garden[i].len() {
            if visited_plots.contains(&[i, j]) {
                continue;
            } else {
                regions.push(map_region_lazy([i, j], &garden, &mut visited_plots));
            }
        }
    }

    Some(regions.iter().fold(0, |acc, (plots, n_sides)| {
        acc + (plots.len() * n_sides) as u32
    }))
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
        assert_eq!(result, Some(1206));
    }
}
