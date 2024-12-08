advent_of_code::solution!(8);

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> ([usize; 2], HashMap<char, Vec<[usize; 2]>>) {
    let n_rows = input.lines().count();
    let n_cols = input.lines().next().unwrap().chars().count();

    let mut freq_pos_map: HashMap<char, Vec<[usize; 2]>> = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        for (j, pos_char) in line.chars().enumerate() {
            if pos_char == '.' {
                continue;
            }

            match freq_pos_map.get_mut(&pos_char) {
                Some(entry) => {
                    entry.push([i, j]);
                }
                None => {
                    let entry_vec = vec![[i, j]];
                    freq_pos_map.insert(pos_char, entry_vec);
                }
            }
        }
    }

    ([n_rows, n_cols], freq_pos_map)
}

fn pos_pair_antinodes(pos_pair: (&[usize; 2], &[usize; 2]), dims: &[usize; 2]) -> Vec<[usize; 2]> {
    let diff: Vec<i32> = pos_pair
        .0
        .iter()
        .zip(pos_pair.1)
        .map(|(a, b)| *a as i32 - *b as i32)
        .collect();

    let potential_pos_arr = [
        [
            pos_pair.0[0] as i32 + diff[0],
            pos_pair.0[1] as i32 + diff[1],
        ],
        [
            pos_pair.1[0] as i32 + diff[0].checked_neg().unwrap(),
            pos_pair.1[1] as i32 + diff[1].checked_neg().unwrap(),
        ],
    ];

    potential_pos_arr
        .iter()
        .map(|[i, j]| [*i as usize, *j as usize])
        .filter(|[i, j]| (*i < dims[0]) && (*j < dims[1]))
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    // - Record the positions for each frequency type in a
    //   HashMap<char, Vec<[usize; 2]>>.
    // - For each pair of positions of one frequency:
    //   - Calculate the new positons as A + (A - B) with A and B substituting
    //     both positions for both A and B.
    //   - If positions are in bounds:
    //     - Save them in a HashSet<[usize; 2]>.
    // - Return the number of elements in the HashSet.
    let (dims, freq_positions) = parse_input(input);

    let mut antinodes: HashSet<[usize; 2]> = HashSet::new();

    for (_, freq_vec) in freq_positions.iter() {
        for pos_pair in freq_vec.iter().combinations(2) {
            let pos_tuple = (pos_pair[0], pos_pair[1]);
            for antinode in pos_pair_antinodes(pos_tuple, &dims) {
                antinodes.insert(antinode);
            }
        }
    }

    Some(antinodes.iter().count() as u32)
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
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
