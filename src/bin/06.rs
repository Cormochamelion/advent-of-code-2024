advent_of_code::solution!(6);

use std::collections::HashSet;

type IterType = std::iter::Cycle<std::vec::IntoIter<(i32, i32)>>;

fn parse_input(input: &str) -> ((usize, usize), Vec<Vec<char>>) {
    let mut output = Vec::new();
    let mut init_pos = (0, 0);
    let init_pos_char = '^';

    for (i, line) in input.lines().enumerate() {
        let mut line_vec = Vec::new();
        for (j, obj_str) in line.chars().enumerate() {
            if obj_str == init_pos_char {
                init_pos = (i, j)
            }

            line_vec.push(obj_str);
        }
        output.push(line_vec);
    }

    return (init_pos, output);
}

struct Guard<'a> {
    curr_pos: (usize, usize),
    curr_dir: (i32, i32),
    visited_positions: HashSet<(usize, usize)>,
    map: &'a Vec<Vec<char>>,
    dimensions: (usize, usize),
    directions_iter: IterType,
}

impl Guard<'_> {
    fn move_with_track(&mut self) -> bool {
        let new_pos = (
            (self.curr_pos.0 as i32 + self.curr_dir.0) as usize,
            (self.curr_pos.1 as i32 + self.curr_dir.1) as usize,
        );

        if !(new_pos.0 < self.dimensions.0 && new_pos.1 < self.dimensions.1) {
            return false;
        }

        if self.map[new_pos.0][new_pos.1] == '#' {
            // A box: Turn 90° right.
            self.curr_dir = self.directions_iter.next().unwrap();
        } else {
            // Free path: Update & log position.
            self.visited_positions.insert(new_pos);
            self.curr_pos = new_pos;
        }

        return true;
    }
}

fn build_guard<'a>(init_pos: (usize, usize), map: &'a Vec<Vec<char>>) -> Guard {
    let visited_set = [init_pos].iter().copied().collect();
    let dimensions = (map.len(), map[0].len());
    let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut dir_iter = directions.into_iter().cycle();

    Guard {
        curr_pos: init_pos,
        curr_dir: dir_iter.next().unwrap(),
        visited_positions: visited_set,
        map: &map,
        dimensions: dimensions,
        directions_iter: dir_iter,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (init_pos, map) = parse_input(input);

    let mut guard = build_guard(init_pos, &map);

    loop {
        // Assuming guard can't walk in circles.
        if !guard.move_with_track() {
            break;
        }
    }

    Some(guard.visited_positions.len() as u32)
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
