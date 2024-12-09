advent_of_code::solution!(9);

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

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
