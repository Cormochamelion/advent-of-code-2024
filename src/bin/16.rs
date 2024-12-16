use std::{cmp, collections::HashSet};

advent_of_code::solution!(16);

fn get_input_dims(input: &str) -> [usize; 2] {
    let mut line_iter = input.lines().peekable();

    let n_cols = line_iter.peek().unwrap().chars().count();
    let n_rows = line_iter.count();

    [n_rows, n_cols]
}

fn parse_input(input: &str) -> ([usize; 2], Vec<Vec<char>>) {
    let [n_rows, n_cols] = get_input_dims(input);

    let mut output = vec![vec!['.'; n_cols]; n_rows];
    let mut init_pos = [0, 0];
    for (i, line) in input.lines().enumerate() {
        for (j, pos_char) in line.chars().enumerate() {
            if pos_char == 'S' {
                init_pos = [i, j]
            }

            output[i][j] = pos_char;
        }
    }

    (init_pos, output)
}

fn get_new_pos(pos: &[usize; 2], movement: &[i32; 2]) -> [usize; 2] {
    pos.iter()
        .zip(movement)
        .map(|(p, m)| (*p as i32 + m) as usize)
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap()
}

fn get_right_angle_dirs(dir: &[i32; 2]) -> [[i32; 2]; 2] {
    match dir {
        [0, 1] | [0, -1] => [[-1, 0], [1, 0]],
        [-1, 0] | [1, 0] => [[0, 1], [0, -1]],
        _ => panic!(
            "Unknown direction {:?}, can't give right angles for that.",
            dir
        ),
    }
}

fn get_new_heads(
    pos: &[usize; 2],
    maze: &[Vec<char>],
    dir: &[i32; 2],
    visited: &HashSet<[usize; 2]>,
) -> [Option<([usize; 2], [i32; 2])>; 3] {
    let mut output = [None; 3];
    let mut new_pos: [usize; 2];

    let [ra_dir_a, ra_dir_b] = get_right_angle_dirs(dir);

    for (i, possible_dir) in [*dir, ra_dir_a, ra_dir_b].iter().enumerate() {
        new_pos = get_new_pos(pos, &possible_dir);

        if maze[new_pos[0]][new_pos[1]] != '#' && !visited.contains(&new_pos) {
            output[i] = Some((new_pos, *possible_dir));
        }
    }

    output
}

fn find_fastest_path(
    init_pos: &[usize; 2],
    maze: &[Vec<char>],
    init_dir: &[i32; 2],
) -> Option<u32> {
    let mut heads: Vec<([usize; 2], [i32; 2], u32, u32)> =
        vec![(*init_pos, *init_dir, 0, 0)];
    let mut visited = HashSet::from([*init_pos]);

    let mut head_pos: [usize; 2];
    let mut head_dir: [i32; 2];
    let mut is_forward_head: bool;

    let mut timer_init: u32;
    let mut value_add: u32;

    let mut i: usize;

    while !heads.is_empty() {
        i = 0;
        while i < heads.len() {
            if heads[i].2 > 0 {
                heads[i].2 -= 1;
                i += 1;
            } else {
                is_forward_head = true;
                for possible_head in
                    get_new_heads(&heads[i].0, maze, &heads[i].1, &visited)
                {
                    if is_forward_head {
                        timer_init = 0;
                        value_add = 1;

                        is_forward_head = false;
                    } else {
                        timer_init = 1000;
                        value_add = 1001;
                    }

                    (head_pos, head_dir) = match possible_head {
                        Some(head) => head,
                        None => continue,
                    };

                    if maze[head_pos[0]][head_pos[1]] == 'E' {
                        return Some(heads[i].3 + value_add);
                    } else {
                        // We already know this is not '#', so it must be '.'.
                        visited.insert(head_pos);

                        heads.push((
                            head_pos,
                            head_dir,
                            timer_init,
                            heads[i].3 + value_add,
                        ))
                    }
                }

                heads.remove(i);
            }
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    // - Initialize a Vec of head positions with direction and timer H with the
    //   start pos, direction, and a timer of 0 and a value of 0.
    // - Initialize a Set of visited positions V.
    // - For each pos in H:
    //    - If timer > 0: Decrement by one.
    //    - Else:
    //     - Find possible new heads (front / left / right) (not '#', not already in H).
    //       - If the new position is 'E': Return the value + 1.
    //       - Else:
    //         - Add the position to V, and to H (timer = 0 if front, else 1000,
    //           value = value of pos + 1 if front, + 1001 else)
    //     - Remove pos from H.
    let (init_pos, maze) = parse_input(input);
    let scores = find_fastest_path(&init_pos, &maze, &[0, 1]);
    let best_score = scores.iter().reduce(|a, b| cmp::min(a, b)).unwrap();

    Some((*best_score) as u32)
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
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
