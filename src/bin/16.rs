use std::{
    cmp,
    collections::{HashMap, HashSet, VecDeque},
};

advent_of_code::solution!(16);

type Node = (Option<Vec<usize>>, ([usize; 2], bool));
type Head = ([usize; 2], [i32; 2], u32, u32, usize, bool);

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
    [
        ((pos[0] as i32) + movement[0]) as usize,
        ((pos[1] as i32) + movement[1]) as usize,
    ]
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
        new_pos = get_new_pos(pos, possible_dir);

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

fn walk_to_root(node: &Node, graph: &[Node]) -> HashSet<[usize; 2]> {
    let mut output = HashSet::new();
    let mut curr_nodes = VecDeque::from([node]);

    let mut curr_node: &Node;

    while !curr_nodes.is_empty() {
        curr_node = curr_nodes.pop_front().unwrap();

        if curr_node.1 .1 {
            output.insert(curr_node.1 .0);
        }

        match curr_node.0.as_ref() {
            None => {
                continue;
            }
            Some(parent_indices) => {
                for idx in parent_indices {
                    curr_nodes.push_back(&graph[*idx]);
                }
            }
        }
    }

    output
}

fn find_fastest_paths(
    init_pos: &[usize; 2],
    maze: &[Vec<char>],
    init_dir: &[i32; 2],
) -> HashSet<[usize; 2]> {
    // Position, direction, timer, index in graph, is_turn.
    let mut head_queue: Vec<Head> =
        Vec::with_capacity(maze.len() * maze[0].len());

    let mut graph: Vec<Node> =
        Vec::with_capacity(maze.len() * maze[0].len() * 2);

    // Maps pos to its node index & serves as a set of visited nodes.
    let mut visited: HashMap<(usize, usize, bool), usize> =
        HashMap::with_capacity(maze.len() * maze[0].len());

    let mut next_head_pos: [usize; 2];
    let mut curr_node_idx: usize;

    let (
        mut curr_head_pos,
        mut curr_head_dir,
        _,
        mut curr_head_val,
        mut curr_head_node_idx,
        mut was_turn,
    ): Head;

    let mut curr_head_idx: usize;

    head_queue.push((*init_pos, *init_dir, 0, 0, graph.len(), false));
    graph.push((None, (*init_pos, true)));
    visited.insert((init_pos[0], init_pos[1], init_dir[0] == 0), 0);

    while !head_queue.is_empty() {
        curr_head_idx = 0;
        while curr_head_idx < head_queue.len() {
            if head_queue[curr_head_idx].2 > 0 {
                head_queue[curr_head_idx].2 -= 1;
                curr_head_idx += 1;
            } else {
                (
                    curr_head_pos,
                    curr_head_dir,
                    _,
                    curr_head_val,
                    curr_head_node_idx,
                    was_turn,
                ) = head_queue[curr_head_idx];

                head_queue.remove(curr_head_idx);

                next_head_pos = get_new_pos(&curr_head_pos, &curr_head_dir);

                if !was_turn {
                    for new_dir in get_right_angle_dirs(&curr_head_dir) {
                        // Push current pos with new direction onto todo
                        // queue.
                        head_queue.push((
                            curr_head_pos,
                            new_dir,
                            1000,
                            curr_head_val + 1000,
                            graph.len(),
                            true,
                        ));

                        graph.push((
                            Some(vec![curr_head_node_idx]),
                            (next_head_pos, false),
                        ));
                    }
                }

                curr_node_idx = graph.len();

                if maze[next_head_pos[0]][next_head_pos[1]] != '#' {
                    graph.push((
                        Some(vec![curr_head_node_idx]),
                        (next_head_pos, true),
                    ));

                    let next_pos_over =
                        get_new_pos(&next_head_pos, &curr_head_dir);

                    if let Some(turn_head) = head_queue.iter().find(|head| {
                        // Handle case joining a head at the current pos.
                        head.2 < 2
                            && head.0 == next_head_pos
                            && head.1 == curr_head_dir
                            && head.3 == curr_head_val + 1
                            && head.5
                    }) {
                        graph[turn_head.4]
                            .0
                            .as_mut()
                            .unwrap()
                            .push(curr_head_node_idx)
                    } else if !visited.contains_key(&(
                        next_head_pos[0],
                        next_head_pos[1],
                        curr_head_dir[0] == 0,
                    )) || head_queue.iter().any(|head| {
                        head.2 > 1
                            && head.0 == next_pos_over
                            && head.1 == curr_head_dir
                            && head.3 >= curr_head_val + 1001
                    }) {
                        // Push forward pos onto todo queue.
                        head_queue.push((
                            next_head_pos,
                            curr_head_dir,
                            1,
                            curr_head_val + 1,
                            curr_node_idx,
                            false,
                        ));

                        visited.insert(
                            (
                                next_head_pos[0],
                                next_head_pos[1],
                                curr_head_dir[0] == 0,
                            ),
                            curr_node_idx,
                        );

                        if maze[next_head_pos[0]][next_head_pos[1]] == 'E' {
                            return walk_to_root(
                                &(
                                    Some(vec![curr_head_node_idx]),
                                    (next_head_pos, true),
                                ),
                                &graph,
                            );
                        }
                    }
                }
            }
        }
    }

    HashSet::new()
}

fn display_maze_paths(pos_set: &HashSet<[usize; 2]>, map: &[Vec<char>]) {
    for (i, row) in map.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if pos_set.contains(&[i, j]) {
                print!("O");
            } else {
                print!("{}", char);
            }
        }
        println!();
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    // Do as in part one, but keep track of the positions visited and return
    // them instead of the length. Submit the number of unique positions.
    // Simple right?
    let (init_pos, maze) = parse_input(input);
    let fastest_paths = find_fastest_paths(&init_pos, &maze, &[0, 1]);

    display_maze_paths(&fastest_paths, &maze);

    Some(fastest_paths.len() as u32)
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

    #[test]
    fn test_part_two_small() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(45));
    }

    #[test]
    fn test_part_two_cross_over() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two_join_on_waiting() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_two_illegal_joining() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(52));
    }

    #[test]
    fn test_part_two_complex_interaction() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 6,
        ));
        assert_eq!(result, Some(32));
    }
}
