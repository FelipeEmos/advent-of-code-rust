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

fn get_neighbors(
    matrix: &Vec<Vec<char>>,
    i: isize,
    initial_j: isize,
    final_j: isize,
) -> Vec<&char> {
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
        .filter_map(|(row, col)| matrix.safe_get(*row, *col))
        .collect();
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
            } else {
                if let Some(initial_j) = str_initial_j {
                    let neighbors =
                        get_neighbors(&matrix, i as isize, initial_j as isize, j as isize - 1);

                    let is_special_part_number = neighbors
                        .iter()
                        .filter(|&c| !c.is_numeric() && **c != '.')
                        .count()
                        > 0;

                    if is_special_part_number {
                        part_numbers.push(number_str.parse::<u32>().unwrap())
                    }

                    str_initial_j = None;
                    number_str.clear();
                }
            }
        }

        if let Some(initial_j) = str_initial_j {
            let neighbors = get_neighbors(
                &matrix,
                i as isize,
                initial_j as isize,
                matrix[i].len() as isize - 1,
            );

            let is_special_part_number = neighbors
                .iter()
                .filter(|&c| !c.is_numeric() && **c != '.')
                .count()
                > 0;

            if is_special_part_number {
                part_numbers.push(number_str.parse::<u32>().unwrap())
            }
        }
    }

    let sum: u32 = part_numbers.iter().sum();

    // println!("PARTS: {:?}", part_numbers);
    // println!("SUM: {:?}", sum);

    Some(sum)
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
