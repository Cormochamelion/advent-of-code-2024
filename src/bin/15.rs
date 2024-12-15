use std::collections::HashSet;

use itertools::enumerate;

advent_of_code::solution!(15);

fn char_to_movement(input: char) -> [i32; 2] {
    match input {
        'v' => [1, 0],
        '>' => [0, 1],
        '^' => [-1, 0],
        '<' => [0, -1],
        _ => panic!("Unknown movement char: {}", input),
    }
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, [usize; 2], Vec<[i32; 2]>) {
    let mut map = Vec::new();
    let mut movements = Vec::new();

    let mut init_pos = [0, 0];

    let mut i: usize = 0;

    let mut is_map = true;

    for line in input.lines() {
        if line.trim() == "" {
            is_map = false;
            continue;
        }
        if is_map {
            let mut row = Vec::new();
            for (j, obj) in enumerate(line.chars()) {
                if obj == '@' {
                    init_pos = [i, j];
                    row.push('.');
                } else {
                    row.push(obj);
                }
            }
            map.push(row);
            i += 1;
        } else {
            for c in line.chars() {
                movements.push(char_to_movement(c));
            }
        }
    }

    (map, init_pos, movements)
}

fn get_new_pos(pos: [usize; 2], movement: [i32; 2]) -> [usize; 2] {
    pos.iter()
        .zip(movement)
        .map(|(p, m)| (*p as i32 + m) as usize)
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap()
}

fn execute_movement(
    movement: &[i32; 2],
    pos: &mut [usize; 2],
    map: &mut [Vec<char>],
) {
    let mut new_pos = get_new_pos(*pos, *movement);
    let mut new_pos_obj = map[new_pos[0]][new_pos[1]];

    match new_pos_obj {
        '.' => *pos = new_pos,
        'O' => {
            let first_box_pos = new_pos;

            // Walk in the movement direction until we encounter a wall or
            // a free space. In the latter case, swap the original box onto
            // the free space.
            while new_pos_obj != '#' {
                if new_pos_obj == '.' {
                    map[new_pos[0]][new_pos[1]] = 'O';
                    map[first_box_pos[0]][first_box_pos[1]] = '.';
                    *pos = first_box_pos;
                    break;
                } else {
                    new_pos = get_new_pos(new_pos, *movement);
                    new_pos_obj = map[new_pos[0]][new_pos[1]];
                }
            }
        }
        '#' => (),
        _ => panic!(
            "Unexpected object in map at position ({}, {}): {}",
            new_pos[0], new_pos[1], new_pos_obj
        ),
    }
}

fn get_box_coords_sum(map: &[Vec<char>]) -> u32 {
    let mut coord_sum = 0;
    // Since there is a wall around the map, we don't need to look at the outer
    // fields.
    for i in 1..(map.len() - 1) {
        for j in 1..(map.len() - 1) {
            if map[i][j] == 'O' {
                coord_sum += (100 * i + j) as u32;
            }
        }
    }

    coord_sum
}

fn display_pos(pos: &[usize; 2], map: &[Vec<char>]) {
    for (i, row) in enumerate(map) {
        if i == pos[0] {
            print!("\"");
            for (j, char) in enumerate(row) {
                if j == pos[1] {
                    print!("@");
                } else {
                    print!("{}", char);
                }
            }
            println!("\"");
        } else {
            println!("{:?}", row.iter().collect::<String>());
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut map, mut pos, movements) = parse_input(input);

    for movement in movements {
        execute_movement(&movement, &mut pos, &mut map);
    }

    Some(get_box_coords_sum(&map))
}

fn parse_input_wide(
    input: &str,
) -> (Vec<Vec<char>>, [usize; 2], Vec<[i32; 2]>) {
    let mut map = Vec::new();
    let mut movements = Vec::new();

    let mut init_pos = [0, 0];

    let mut i: usize = 0;

    let mut is_map = true;

    for line in input.lines() {
        if line.trim() == "" {
            is_map = false;
            continue;
        }
        if is_map {
            let mut row = Vec::new();
            for (j, obj) in enumerate(line.chars()) {
                if obj == '@' {
                    init_pos = [i, j * 2];
                    row.push('.');
                    row.push('.');
                } else if obj == 'O' {
                    row.push('[');
                    row.push(']');
                } else {
                    row.push(obj);
                    row.push(obj);
                }
            }
            map.push(row);
            i += 1;
        } else {
            for c in line.chars() {
                movements.push(char_to_movement(c));
            }
        }
    }

    (map, init_pos, movements)
}

fn get_full_box_pos(pos: &[usize; 2], map: &[Vec<char>]) -> [[usize; 2]; 2] {
    let complement_pos = match map[pos[0]][pos[1]] {
        '[' => get_new_pos(*pos, [0, 1]),
        ']' => get_new_pos(*pos, [0, -1]),
        _ => panic!(
            "The handle of the box needs to be one of ('[', ']'), not '{}'",
            map[pos[0]][pos[1]]
        ),
    };

    [*pos, complement_pos]
}

fn move_box(movement: &[i32; 2], pos: &[usize; 2], map: &mut [Vec<char>]) {
    let [handle_pos, complement_pos] = get_full_box_pos(pos, map);

    let [new_handle_pos, new_complement_pos]: [[usize; 2]; 2] =
        [handle_pos, complement_pos]
            .iter()
            .map(|pos| get_new_pos(*pos, *movement))
            .collect::<Vec<[usize; 2]>>()
            .try_into()
            .unwrap();

    let handle_char = map[handle_pos[0]][handle_pos[1]];
    let compl_char = map[complement_pos[0]][complement_pos[1]];

    map[new_handle_pos[0]][new_handle_pos[1]] = handle_char;
    map[new_complement_pos[0]][new_complement_pos[1]] = compl_char;

    if movement[0] != 0 {
        map[pos[0]][pos[1]] = '.';
        map[complement_pos[0]][complement_pos[1]] = '.';
    } else {
        map[pos[0]][pos[1]] = '.';
    }
}

fn find_boxes_in_dir(
    pos: &[usize; 2],
    map: &[Vec<char>],
    dir: &[i32; 2],
) -> Option<Vec<[[usize; 2]; 2]>> {
    let [handle_pos, compl_pos] = get_full_box_pos(pos, map);

    let [new_handle_pos, new_compl_pos]: [[usize; 2]; 2] = [*pos, compl_pos]
        .iter()
        .map(|pos| get_new_pos(*pos, *dir))
        .collect::<Vec<[usize; 2]>>()
        .try_into()
        .unwrap();

    let new_handle_char = map[new_handle_pos[0]][new_handle_pos[1]];
    let new_compl_char = map[new_compl_pos[0]][new_compl_pos[1]];

    if new_handle_char == '#' || new_compl_char == '#' {
        return None;
    } else {
        let mut output = Vec::new();

        if new_handle_char != '.' {
            match find_boxes_in_dir(&new_handle_pos, map, dir) {
                Some(mut boxes) => output.append(&mut boxes),
                None => return None,
            }
        }

        // If the new complementary char is the same as the current complementary
        // char and the box below would the the same we added for the handle above.
        if new_compl_char != '.'
            && new_compl_char != map[compl_pos[0]][compl_pos[1]]
        {
            match find_boxes_in_dir(&new_compl_pos, map, dir) {
                Some(mut boxes) => output.append(&mut boxes),
                None => return None,
            }
        }

        if handle_pos[1] < compl_pos[1] {
            output.push([handle_pos, compl_pos]);
        } else {
            output.push([compl_pos, handle_pos]);
        }

        return Some(output);
    }
}

fn deconstruct_box(wide_box: &[[usize; 2]; 2]) -> [usize; 4] {
    let [[a, b], [c, d]] = *wide_box;
    [a, b, c, d]
}

fn movement_with_boxes(
    movement: &[i32; 2],
    pos: &mut [usize; 2],
    new_pos: [usize; 2],
    map: &mut [Vec<char>],
) {
    let mut new_pos_obj = map[new_pos[0]][new_pos[1]];
    let mut new_pos = new_pos;

    if movement[0] == 0 {
        let first_box_pos = new_pos;

        // Check in the movement direction until we encounter a wall or
        // a free space. In the latter case, move all boxes in between
        // one step in the direction.
        while new_pos_obj != '#' {
            if new_pos_obj == '.' {
                // Create an iterator over row indices from the last
                // box next to the free space to the box next to the
                // robot to move each box towards the free space.let
                let mut ind_offset = 2;

                if movement[1].is_negative() {
                    ind_offset = -2;
                }

                let ind_last_box = (new_pos[1] as i32 - ind_offset) as usize;

                if movement[1].is_negative() {
                    for j in (ind_last_box..=first_box_pos[1]).step_by(2) {
                        move_box(movement, &[first_box_pos[0], j], map)
                    }
                } else {
                    for j in (first_box_pos[1]..=ind_last_box).rev().step_by(2)
                    {
                        move_box(movement, &[first_box_pos[0], j], map)
                    }
                }

                *pos = first_box_pos;
                break;
            } else {
                new_pos = get_new_pos(new_pos, *movement);
                new_pos_obj = map[new_pos[0]][new_pos[1]];
            }
        }
    } else {
        let boxes = match find_boxes_in_dir(&new_pos, map, movement) {
            Some(boxes) => boxes,
            None => return (),
        };

        let mut unique_boxes: HashSet<[usize; 4]> =
            boxes.iter().map(deconstruct_box).collect();

        for wide_box in boxes {
            let box_representation = deconstruct_box(&wide_box);
            if unique_boxes.contains(&box_representation) {
                move_box(movement, &wide_box[0], map);
                unique_boxes.remove(&box_representation);
            }
        }

        *pos = new_pos;
    }
}

fn execute_movement_wide(
    movement: &[i32; 2],
    pos: &mut [usize; 2],
    map: &mut [Vec<char>],
) {
    let new_pos = get_new_pos(*pos, *movement);
    let new_pos_obj = map[new_pos[0]][new_pos[1]];

    match new_pos_obj {
        '.' => *pos = new_pos,
        '#' => (),
        _ => movement_with_boxes(movement, pos, new_pos, map),
    }
}

fn get_wide_box_coords_sum(map: &[Vec<char>]) -> u32 {
    let mut coord_sum = 0;

    // Since there is a wall around the map, we don't need to look at the outer
    // fields.
    for i in 1..=(map.len() - 1) {
        for j in 1..=(map[0].len() - 1) {
            if map[i][j] == '[' {
                coord_sum += (100 * i + j) as u32;
            }
        }
    }

    coord_sum
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut map, mut pos, movements) = parse_input_wide(input);

    for movement in movements {
        execute_movement_wide(&movement, &mut pos, &mut map);
    }
    display_pos(&pos, &map);

    Some(get_wide_box_coords_sum(&map))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
