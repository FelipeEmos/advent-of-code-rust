advent_of_code::solution!(1);

fn get_line_value(line: &str, digit_values_table: Vec<(&str, u32)>) -> u32 {
    let numbers = (0..line.len())
        .filter_map(|i| {
            for (digit, value) in digit_values_table.iter() {
                if (&line[i..]).starts_with(digit) {
                    return Some(*value);
                }
            }
            None
        })
        .collect::<Vec<_>>();

    let first_digit = numbers.first().unwrap();
    let last_digit = numbers.last().unwrap();

    return format!("{}{}", first_digit, last_digit)
        .parse::<u32>()
        .unwrap();
}

pub fn part_one(input: &str) -> Option<u32> {
    let total_sum: u32 = input
        .lines()
        .map(|line| {
            get_line_value(
                line,
                vec![
                    ("0", 0),
                    ("1", 1),
                    ("2", 2),
                    ("3", 3),
                    ("4", 4),
                    ("5", 5),
                    ("6", 6),
                    ("7", 7),
                    ("8", 8),
                    ("9", 9),
                ],
            )
        })
        .sum();

    Some(total_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let total_sum: u32 = input
        .lines()
        .map(|line| {
            get_line_value(
                line,
                vec![
                    ("0", 0),
                    ("1", 1),
                    ("2", 2),
                    ("3", 3),
                    ("4", 4),
                    ("5", 5),
                    ("6", 6),
                    ("7", 7),
                    ("8", 8),
                    ("9", 9),
                    // Literal numbers
                    ("zero", 0),
                    ("one", 1),
                    ("two", 2),
                    ("three", 3),
                    ("four", 4),
                    ("five", 5),
                    ("six", 6),
                    ("seven", 7),
                    ("eight", 8),
                    ("nine", 9),
                ],
            )
        })
        .sum();

    Some(total_sum)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_part_one() {
//         let result = part_one(&advent_of_code::template::read_file("examples", DAY));
//         assert_eq!(result, None);
//     }
//
//     #[test]
//     fn test_part_two() {
//         let result = part_two(&advent_of_code::template::read_file("examples", DAY));
//         assert_eq!(result, None);
//     }
// }
