advent_of_code::solution!(9);

use std::collections::{HashMap, VecDeque};

fn parse_input(input: &str) -> Vec<Option<usize>> {
    let trim_input = input.trim_end();
    let pos_lens: Vec<usize> = trim_input
        .chars()
        .map(|pos_char| pos_char.to_digit(10).unwrap() as usize)
        .collect();
    let vec_len: usize = pos_lens.iter().sum();
    let mut disk_map: Vec<Option<usize>> = vec![None; vec_len];

    let mut is_file = true;
    let mut file_id: usize = 0;
    let mut i: usize = 0;

    for pos_len in pos_lens {
        if is_file {
            for _ in 0..pos_len {
                disk_map[i] = Some(file_id);
                i += 1;
            }

            is_file = false;
            file_id += 1;
        } else {
            i += pos_len;

            is_file = true;
        }
    }

    disk_map
}

fn compress_disk(disk_map: &mut Vec<Option<usize>>) -> u64 {
    let mut i: usize = 0;
    let mut j: usize = disk_map.len() - 1;
    let mut checksum: usize = 0;

    loop {
        if disk_map[i] == None && j > i {
            // If we crossed i over j, we don't want to do any swaps any more.
            disk_map.swap(i, j);

            while disk_map[j] == None {
                j -= 1;
            }
        }

        match disk_map[i] {
            Some(file_id) => {
                checksum += file_id * i;
            }
            None => {
                // We didn't swap and have reached the end of files on disk.
                // It's only free space beyond here.
                break;
            }
        }

        i += 1;
    }

    checksum.try_into().unwrap()
}

pub fn part_one(input: &str) -> Option<u64> {
    // - Parse input:
    //   - Create a vector of Option<usiz> D.
    //   - Create file-id index x: usize.
    //   - Create is_file = true.
    //   - Trim whitespace from input.
    //   - For each char c:
    //     - Parse c to u32.
    //     - If is_file:
    //       - Push Option(x) onto D c times.
    //       - Increment x.
    //       - Toggle is_file.
    //     - Else:
    //       - Push None onto D c times.
    //       - Toggle is_file.
    // - Compress used space:
    //   - Create front index i: usize = 0 and tail index j: usize = len(D).
    //   - Create checksum: u32 = 0.
    //   - Loop:
    //     - If D[i] == None and not i => j:
    //       - Swap D[i] & D[j].
    //       - Decrement j until it no longer points at None.
    //     - If D[i] is now not None:
    //       - Increment checksum by D[i].unwrap() * i.
    //     - Else Break.
    //     - Increment i.
    // - Return checksum.
    let mut disk_map = parse_input(input);

    let checksum = compress_disk(&mut disk_map);

    Some(checksum)
}

fn parse_input_contig(
    input: &str,
) -> (
    Vec<Option<usize>>,
    Vec<(usize, usize)>,
    HashMap<usize, VecDeque<usize>>,
) {
    let trim_input = input.trim_end();
    let pos_lens: Vec<usize> = trim_input
        .chars()
        .map(|pos_char| pos_char.to_digit(10).unwrap() as usize)
        .collect();
    let vec_len: usize = pos_lens.iter().sum();
    let mut disk_map: Vec<Option<usize>> = vec![None; vec_len];
    let mut files: Vec<(usize, usize)> = Vec::new();
    let mut free_space: HashMap<usize, VecDeque<usize>> = HashMap::new();

    let mut is_file = true;
    let mut file_id: usize = 0;
    let mut file_pos: usize;
    let mut i: usize = 0;

    for pos_len in pos_lens {
        if is_file {
            file_pos = i.clone();
            for _ in 0..pos_len {
                disk_map[i] = Some(file_id);
                i += 1;
            }

            files.push((file_pos, pos_len.clone()));

            is_file = false;
            file_id += 1;
        } else {
            match free_space.get_mut(&pos_len) {
                Some(entry) => entry.push_back(i.clone()),
                None => _ = free_space.insert(pos_len.clone(), VecDeque::from([i.clone()])),
            };

            i += pos_len;

            is_file = true;
        }
    }

    (disk_map, files, free_space)
}

fn find_space(
    size: &usize,
    max_pos: &usize,
    space_map: &mut HashMap<usize, VecDeque<usize>>,
    max_free_space: usize,
) -> Option<(usize, usize)> {
    let mut leftest = max_pos.clone();
    let mut space_tup: Option<(usize, usize)> = None;

    for size in *size..=max_free_space {
        let size_min_pos = match space_map.get_mut(&size).and_then(|entry| entry.front()) {
            Some(&size_min_pos) => size_min_pos,
            None => continue,
        };

        if size_min_pos < leftest {
            leftest = size_min_pos;
            space_tup = Some((size, size_min_pos));
        }
    }

    match space_tup {
        Some((size, _)) => {
            _ = space_map.get_mut(&size).unwrap().pop_front();
        }
        None => {}
    }

    space_tup
}

fn insert_new_space(size: usize, pos: usize, free_space: &mut HashMap<usize, VecDeque<usize>>) {
    match free_space.get_mut(&size) {
        Some(entry) => {
            let mut i: usize = 0;
            for el in entry.iter() {
                if pos < *el {
                    break;
                } else {
                    i += 1;
                }
            }
            entry.insert(i, pos);
        }
        None => _ = free_space.insert(pos, VecDeque::from([size])),
    };
}

fn compress_disk_contig(
    disk_map: &mut Vec<Option<usize>>,
    files: Vec<(usize, usize)>,
    free_space: &mut HashMap<usize, VecDeque<usize>>,
) -> u64 {
    let max_free_space: usize = *free_space.keys().max().unwrap();
    let mut i: usize;
    let mut j: usize;
    let mut new_space_size: usize;

    for (file_pos, file_len) in files.iter().rev() {
        let (space_size, space_pos) =
            match find_space(file_len, file_pos, free_space, max_free_space) {
                Some(space_data) => space_data,
                None => continue,
            };

        i = file_pos.clone();
        j = space_pos;

        for _ in *file_pos..(file_pos + file_len) {
            disk_map.swap(i, j);

            i += 1;
            j += 1;
        }

        new_space_size = space_size - file_len;

        if new_space_size > 0 {
            insert_new_space(new_space_size, j, free_space);
        }
    }

    let raw_count = disk_map
        .iter()
        .enumerate()
        .fold(0, |acc: usize, (i, block)| match block {
            Some(file_id) => acc + (file_id * i),
            None => acc,
        });

    raw_count.try_into().unwrap()
}

pub fn part_two(input: &str) -> Option<u64> {
    // - Parse input:
    //   - Create files: Vec<(int, int)> of (position, length).
    //   - Create free_space: HashMap<int, Queue<int>> mapping free space size
    //     to positions with that amount.
    //   - Iterate over input chars like in part 1. Additionally:
    //     - If is_file, fill files.
    //     - Else fill free_space.
    // - Compress used space without fragmentation:
    //   - Create max_free_space = max of keys in free_space (Throughout the
    //     algorithm, we will never have more space available since we
    //     progressively fill any space to the left of the current file, never
    //     looking right of it.).
    //   - For file_id in decreasingly sorted keys of files:
    //     - Find free space in free_space by checking free_space in increasing
    //       key size from size of file to max_free_space.
    //     - If none found, continue.
    //     - Else:
    //       - Get free space pos i and file position j.
    //       - For _ in range(file size) swap D[i] and D[j].
    //       - Update free_space by calculating the new size as
    //         og_size - file_size, removing the original entry and adding
    //         a new one at the new file size.
    //  - Compute the checksum by iterating through the modified disk map.
    //  - Return checksum.
    let (mut disk_map, files, mut free_space) = parse_input_contig(input);
    let checksum = compress_disk_contig(&mut disk_map, files, &mut free_space);

    Some(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
