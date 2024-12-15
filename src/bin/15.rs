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
    display_pos(&pos, &map);

    Some(get_box_coords_sum(&map))
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
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
