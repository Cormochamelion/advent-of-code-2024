advent_of_code::solution!(10);

use std::collections::HashSet;

const DIRECTIONS: [[i32; 2]; 4] = [[-1, 0], [0, -1], [0, 1], [1, 0]];

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let mut output = Vec::new();

    for line in input.lines() {
        output.push(
            line.chars()
                .map(|height_char| height_char.to_digit(10).unwrap())
                .collect::<Vec<u32>>(),
        );
    }

    output
}

fn count_peaks_from_trailhead(
    map: &Vec<Vec<u32>>,
    trail_head_pos: Vec<usize>,
    visited_peaks: &mut HashSet<(usize, usize)>,
) -> u32 {
    let mut trail_head_score = 0;
    let dims = vec![map.len(), map[0].len()];
    let mut new_pos: Vec<usize>;
    let mut new_height: u32;

    let curr_height = map[trail_head_pos[0]][trail_head_pos[1]];

    if curr_height == 9 {
        let trail_head_tup = (trail_head_pos[0], trail_head_pos[1]);
        match visited_peaks.get(&trail_head_tup) {
            Some(_) => return 0,
            None => {
                visited_peaks.insert(trail_head_tup);
                return 1;
            }
        }
    }

    for dir in DIRECTIONS.iter().map(|dir| Vec::from(dir)) {
        new_pos = trail_head_pos
            .iter()
            .zip(dir)
            .map(|(coord, diff)| (*coord as i32 + diff) as usize)
            .collect();

        if new_pos[0] >= dims[0] || new_pos[1] >= dims[1] {
            continue;
        }

        new_height = map[new_pos[0]][new_pos[1]];

        if new_height.checked_sub(curr_height).unwrap_or(0) != 1 {
            continue;
        }

        trail_head_score += count_peaks_from_trailhead(map, new_pos, visited_peaks);
    }

    trail_head_score
}

pub fn part_one(input: &str) -> Option<u32> {
    // Start from trail head (0).
    // Look around for trail head + 1.
    // Start from there.
    // If trail head is 9 and we haven't visited -> return 1
    let topo_map = parse_input(input);
    let mut trail_head_score = 0;

    for i in 0..topo_map.len() {
        for j in 0..topo_map[i].len() {
            if topo_map[i][j] == 0 {
                let score = count_peaks_from_trailhead(&topo_map, vec![i, j], &mut HashSet::new());
                trail_head_score += score;
            }
        }
    }

    Some(trail_head_score)
}

fn count_trails_from_trailhead(map: &Vec<Vec<u32>>, trail_head_pos: Vec<usize>) -> u32 {
    let mut trail_head_score = 0;
    let dims = vec![map.len(), map[0].len()];
    let mut new_pos: Vec<usize>;
    let mut new_height: u32;

    let curr_height = map[trail_head_pos[0]][trail_head_pos[1]];

    if curr_height == 9 {
        return 1;
    }

    for dir in DIRECTIONS.iter().map(|dir| Vec::from(dir)) {
        new_pos = trail_head_pos
            .iter()
            .zip(dir)
            .map(|(coord, diff)| (*coord as i32 + diff) as usize)
            .collect();

        if new_pos[0] >= dims[0] || new_pos[1] >= dims[1] {
            continue;
        }

        new_height = map[new_pos[0]][new_pos[1]];

        if new_height.checked_sub(curr_height).unwrap_or(0) != 1 {
            continue;
        }

        trail_head_score += count_trails_from_trailhead(map, new_pos);
    }

    trail_head_score
}

pub fn part_two(input: &str) -> Option<u32> {
    // Start from trail head (0).
    // Look around for trail head + 1.
    // Start from there.
    // If trail head is 9 -> return 1
    let topo_map = parse_input(input);
    let mut trail_head_score = 0;

    for i in 0..topo_map.len() {
        for j in 0..topo_map[i].len() {
            if topo_map[i][j] == 0 {
                let score = count_trails_from_trailhead(&topo_map, vec![i, j]);
                trail_head_score += score;
            }
        }
    }

    Some(trail_head_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
