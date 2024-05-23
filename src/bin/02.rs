advent_of_code::solution!(2);

struct GameInfo {
    game_id: u32,
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}

#[derive(Debug)]
enum BallSet {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl BallSet {
    fn new(color: &str, quantity: u32) -> Self {
        match color {
            "red" => BallSet::Red(quantity),
            "green" => BallSet::Green(quantity),
            "blue" => BallSet::Blue(quantity),
            c => panic!("Invalid color {:?}", c),
        }
    }

    fn from_str(str_item: &str) -> Self {
        let mut parts = str_item.trim().split(" ");

        let quantity = parts.next().unwrap().parse::<u32>().unwrap();
        let color = parts.next().unwrap();

        Self::new(color, quantity)
    }
}

fn get_game_info(line: &str) -> GameInfo {
    let mut parts = line.split(&[':', ';'][..]);

    let header = parts.next().unwrap();
    let game_id = header.split(" ").last().unwrap().parse::<u32>().unwrap();

    let items = parts
        .flat_map(|part| part.split(","))
        .map(|item| BallSet::from_str(item))
        .collect::<Vec<_>>();

    let max_used_red = items
        .iter()
        .filter_map(|item| match item {
            BallSet::Red(quantity) => Some(quantity),
            _ => None,
        })
        .copied()
        .max()
        .unwrap_or(0);

    let max_used_green = items
        .iter()
        .filter_map(|item| match item {
            BallSet::Green(quantity) => Some(quantity),
            _ => None,
        })
        .copied()
        .max()
        .unwrap_or(0);

    let max_used_blue = items
        .iter()
        .filter_map(|item| match item {
            BallSet::Blue(quantity) => Some(quantity),
            _ => None,
        })
        .copied()
        .max()
        .unwrap_or(0);

    GameInfo {
        game_id,
        max_red: max_used_red,
        max_green: max_used_green,
        max_blue: max_used_blue,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let total_red = 12;
    let total_green = 13;
    let total_blue = 14;

    let valid_games_id_sum: u32 = input
        .lines()
        .filter_map(|line| {
            let GameInfo {
                game_id,
                max_red,
                max_green,
                max_blue,
            } = get_game_info(line);
            if max_red <= total_red && max_green <= total_green && max_blue <= total_blue {
                return Some(game_id);
            }
            return None;
        })
        .sum();

    println!("RESULT {}", valid_games_id_sum);
    Some(valid_games_id_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let power_games_sum: u32 = input
        .lines()
        .map(|line| {
            let GameInfo {
                max_red,
                max_green,
                max_blue,
                ..
            } = get_game_info(line);

            return max_red * max_green * max_blue;
        })
        .sum();

    println!("RESULT {}", power_games_sum);
    Some(power_games_sum)
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
