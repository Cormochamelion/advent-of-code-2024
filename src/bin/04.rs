advent_of_code::solution!(4);

use nalgebra::DMatrix;

struct Direction {
    offsets: (i32, i32),
}

impl Direction {
    fn move_from(&self, pos: (usize, usize)) -> (i32, i32) {
        let (row_off, col_off) = self.offsets;
        let (row, col) = pos;

        let new_row = row as i32 + row_off;
        let new_col = col as i32 + col_off;

        return (new_row, new_col);
    }
}

static BOTTOM_LEFT: Direction = Direction { offsets: (-1, -1) };
static BOTTOM_RIGHT: Direction = Direction { offsets: (-1, 1) };
static TOP_LEFT: Direction = Direction { offsets: (1, -1) };
static TOP_RIGHT: Direction = Direction { offsets: (1, 1) };

static DIRECTIONS: [&Direction; 8] = [
    &BOTTOM_LEFT,
    &Direction { offsets: (0, -1) },
    &TOP_LEFT,
    &Direction { offsets: (-1, 0) },
    &Direction { offsets: (1, 0) },
    &BOTTOM_RIGHT,
    &Direction { offsets: (0, 1) },
    &TOP_RIGHT,
];

fn input_to_matrix(input: &str) -> DMatrix<char> {
    let mut line_iter = input.lines().peekable();
    let n_chars = line_iter
        .peek()
        .expect("Can't peek input. Is there anything there?")
        .len();

    let n_lines = line_iter.count();

    let clean_input = input.to_owned().replace("\n", "");
    let input_iter = clean_input.chars();

    let row_major_mat = DMatrix::from_iterator(n_chars, n_lines, input_iter);

    return row_major_mat.transpose();
}

fn check_mat_pos(matrix: &DMatrix<char>, pos: (i32, i32)) -> bool {
    let (row, col) = pos;

    let n_rows = matrix.nrows() as i32;
    let n_cols = matrix.ncols() as i32;

    let row_ok = row >= 0 && row < n_rows;

    let col_ok = col >= 0 && col < n_cols;

    return row_ok && col_ok;
}

fn pos_to_indices(pos: (i32, i32)) -> (usize, usize) {
    let (row, col) = pos;

    return (row as usize, col as usize);
}

fn continues_word(
    char_mat: &DMatrix<char>,
    pos: (usize, usize),
    word: &String,
    direction: &Direction,
) -> bool {
    let mut word_iter = word.chars();
    let first_char = word_iter.next().unwrap();

    if char_mat[pos] == first_char {
        let new_pos = direction.move_from(pos);

        let word_remainder = word_iter.collect::<String>();

        if word_remainder.chars().count() < 1 {
            return true;
        }

        if !check_mat_pos(char_mat, new_pos) {
            return false;
        }
        return continues_word(
            char_mat,
            pos_to_indices(new_pos),
            &word_remainder,
            &direction,
        );
    } else {
        return false;
    }
}

fn starts_n_words(char_mat: &DMatrix<char>, pos: (usize, usize), word: String) -> u32 {
    let mut word_iter = word.chars();

    match word_iter.next() {
        Some(first_char) => {
            if char_mat[pos] == first_char {
                let mut n_words = 0;

                let word_remainder = word_iter.collect::<String>();

                if word_remainder.chars().count() < 1 {
                    return 1;
                }

                for direction in DIRECTIONS.iter() {
                    let new_pos = direction.move_from(pos);

                    if !check_mat_pos(char_mat, new_pos) {
                        continue;
                    }

                    if continues_word(
                        char_mat,
                        pos_to_indices(new_pos),
                        &word_remainder,
                        &direction,
                    ) {
                        n_words += 1;
                    }
                }
                return n_words;
            } else {
                return 0;
            }
        }
        // End of the word, we found something.
        _ => {
            return 1;
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let input_mat = input_to_matrix(input);

    let search_word: &str = "XMAS";
    let mut n_words: u32 = 0;

    for i in 0..input_mat.nrows() {
        for j in 0..input_mat.ncols() {
            n_words += starts_n_words(&input_mat, (i, j), String::from(search_word));
        }
    }

    Some(n_words)
}

fn string_from_index_slice(matrix: &DMatrix<char>, index_slice: &[(usize, usize)]) -> String {
    return index_slice
        .iter()
        .map(|slice| matrix[*slice])
        .collect::<String>();
}

fn is_xmas_a(char_mat: &DMatrix<char>, pos: (usize, usize)) -> bool {
    if !(char_mat[pos] == 'A') {
        return false;
    }

    let diag_1 = [
        pos_to_indices(TOP_LEFT.move_from(pos)),
        pos,
        pos_to_indices(BOTTOM_RIGHT.move_from(pos)),
    ];

    let diag_2 = [
        pos_to_indices(TOP_RIGHT.move_from(pos)),
        pos,
        pos_to_indices(BOTTOM_LEFT.move_from(pos)),
    ];

    let all_diags_are_mas = [diag_1, diag_2]
        .into_iter()
        .map(|diag| string_from_index_slice(char_mat, &diag))
        .all(|diag_str| diag_str == "SAM" || diag_str == "MAS");

    return all_diags_are_mas;
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_mat = input_to_matrix(input);

    let mut n_xmas = 0;

    for i in 1..(input_mat.nrows() - 1) {
        for j in 1..(input_mat.ncols() - 1) {
            if is_xmas_a(&input_mat, (i, j)) {
                n_xmas += 1;
            }
        }
    }

    Some(n_xmas)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
