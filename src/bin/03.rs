use std::collections::HashMap;

advent_of_code::solution!(3);

trait MatrixSafeAccess<T> {
    fn safe_get(&self, i: isize, j: isize) -> Option<&T>;
}

impl<T> MatrixSafeAccess<T> for Vec<Vec<T>> {
    fn safe_get(&self, i: isize, j: isize) -> Option<&T> {
        if i < 0 || (i as usize) >= self.len() {
            return None;
        }

        match self.get(i as usize) {
            Some(line) => {
                if j < 0 || (j as usize) >= line.len() {
                    None
                } else {
                    line.get(j as usize)
                }
            }
            _ => None,
        }
    }
}

struct MatrixItem {
    item: char,
    row: isize,
    col: isize,
}

fn get_neighbors(
    matrix: &Vec<Vec<char>>,
    i: isize,
    initial_j: isize,
    final_j: isize,
) -> Vec<MatrixItem> {
    let left = vec![
        (i - 1, initial_j - 1),
        (i, initial_j - 1),
        (i + 1, initial_j - 1),
    ];

    let right = vec![(i - 1, final_j + 1), (i, final_j + 1), (i + 1, final_j + 1)];

    let up = (initial_j..final_j + 1)
        .map(|j| (i - 1, j))
        .collect::<Vec<(isize, isize)>>();

    let down = (initial_j..final_j + 1)
        .map(|j| (i + 1, j))
        .collect::<Vec<(isize, isize)>>();

    // NOTE: probably there's a better way of combining them haha
    let locations = vec![left, up, right, down].concat();

    return locations
        .iter()
        .filter_map(|(row, col)| match matrix.safe_get(*row, *col) {
            Some(c) => Some(MatrixItem {
                item: *c,
                row: *row,
                col: *col,
            }),
            None => None,
        })
        .collect();
}

fn has_part_neighbor(neighbors: &Vec<MatrixItem>) -> bool {
    neighbors
        .iter()
        .any(|MatrixItem { item, .. }| !item.is_numeric() && *item != '.')
}

fn parse_matrix(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let matrix = parse_matrix(input);
    let mut part_numbers: Vec<u32> = Vec::new();

    for i in 0..matrix.len() {
        let mut number_str = String::new();
        let mut str_initial_j: Option<usize> = None;

        for j in 0..matrix[i].len() {
            if matrix[i][j].is_numeric() {
                number_str.push(matrix[i][j]);

                if str_initial_j.is_none() {
                    str_initial_j = Some(j);
                }
            }

            if str_initial_j.is_some() {
                let end_of_line = j == matrix[i].len() - 1;
                let end_of_word = !matrix[i][j].is_numeric();

                if !end_of_line && !end_of_word {
                    continue;
                }

                let initial_j = str_initial_j.unwrap();
                let final_j = if end_of_line { j } else { j - 1 };

                let neighbors =
                    get_neighbors(&matrix, i as isize, initial_j as isize, final_j as isize);

                if has_part_neighbor(&neighbors) {
                    part_numbers.push(number_str.parse::<u32>().unwrap())
                }

                str_initial_j = None;
                number_str.clear();
            }
        }
    }

    let sum: u32 = part_numbers.iter().sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix = parse_matrix(input);
    let mut gear_numbers_sets: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for i in 0..matrix.len() {
        let mut number_str = String::new();
        let mut str_initial_j: Option<usize> = None;

        for j in 0..matrix[i].len() {
            if matrix[i][j].is_numeric() {
                number_str.push(matrix[i][j]);

                if str_initial_j.is_none() {
                    str_initial_j = Some(j);
                }
            }

            if str_initial_j.is_some() {
                let end_of_line = j == matrix[i].len() - 1;
                let end_of_word = !matrix[i][j].is_numeric();

                if !end_of_line && !end_of_word {
                    continue;
                }

                let initial_j = str_initial_j.unwrap();
                let final_j = if end_of_line { j } else { j - 1 };

                let neighbors =
                    get_neighbors(&matrix, i as isize, initial_j as isize, final_j as isize);

                let current_number = number_str.parse::<u32>().unwrap();

                neighbors
                    .iter()
                    .filter(|MatrixItem { item, .. }| *item == '*')
                    .for_each(|MatrixItem { row, col, .. }| {
                        gear_numbers_sets
                            .entry((*row as usize, *col as usize))
                            .and_modify(|v| v.push(current_number))
                            .or_insert(vec![current_number]);
                    });

                str_initial_j = None;
                number_str.clear();
            }
        }
    }

    let sum: u32 = gear_numbers_sets
        .values()
        .filter(|v| v.len() > 1)
        .map(|v| v.iter().fold(1, |acc, x| acc * x))
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
