use std::cmp;

use regex::Regex;

advent_of_code::solution!(2);
pub fn part_one(input: &str) -> Option<u32> {
    let EXPECTED_RED = 12;
    let EXPECTED_GREEN = 13;
    let EXPECTED_BLUE = 14;

    let mut game_id_sum = 0;
    let lines = input.lines();
    for line in lines {
        let game_id_chunks: Vec<&str> = line.split(":").collect();
        let game_id_chunk: Vec<&str> = game_id_chunks[0].split(" ").collect();
        let game_id: u32 = game_id_chunk[1].parse().unwrap();
        let showings_chunks: Vec<&str> = game_id_chunks[1].split(";").collect();
        let mut skip = false;
        for chunk in showings_chunks {
            let red_regexp = Regex::new(r"(\d+) red").unwrap();
            let green_regexp = Regex::new(r"(\d+) green").unwrap();
            let blue_regexp = Regex::new(r"(\d+) blue").unwrap();

            if let Some(matches) = red_regexp.captures(chunk) {
                if let Some(count_match) = matches.get(1) {
                    let count: u32 = count_match.as_str().parse().unwrap();
                    if count > EXPECTED_RED {
                        skip = true;
                    }
                }
            }
            if let Some(matches) = green_regexp.captures(chunk) {
                if let Some(count_match) = matches.get(1) {
                    let count: u32 = count_match.as_str().parse().unwrap();
                    if count > EXPECTED_GREEN {
                        skip = true;
                    }
                }
            }

            if let Some(matches) = blue_regexp.captures(chunk) {
                if let Some(count_match) = matches.get(1) {
                    let count: u32 = count_match.as_str().parse().unwrap();
                    if count > EXPECTED_BLUE {
                        skip = true;
                    }
                }
            }
        }

        if skip {
            continue;
        }
        game_id_sum += game_id;
    }
    return Some(game_id_sum);
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut total_sum = 0;
    for line in lines {
        let game_id_chunks: Vec<&str> = line.split(":").collect();
        let game_id_chunk: Vec<&str> = game_id_chunks[0].split(" ").collect();
        let showings_chunks: Vec<&str> = game_id_chunks[1].split(";").collect();

        let mut min_red = 1;
        let mut min_green = 1;
        let mut min_blue = 1;
        for chunk in showings_chunks {
            let red_regexp = Regex::new(r"(\d+) red").unwrap();
            let green_regexp = Regex::new(r"(\d+) green").unwrap();
            let blue_regexp = Regex::new(r"(\d+) blue").unwrap();

            if let Some(matches) = red_regexp.captures(chunk) {
                if let Some(count_match) = matches.get(1) {
                    let count: u32 = count_match.as_str().parse().unwrap();
                    min_red = cmp::max(min_red, count)
                }
            }
            if let Some(matches) = green_regexp.captures(chunk) {
                if let Some(count_match) = matches.get(1) {
                    let count: u32 = count_match.as_str().parse().unwrap();
                    min_green = cmp::max(min_green, count)
                }
            }

            if let Some(matches) = blue_regexp.captures(chunk) {
                if let Some(count_match) = matches.get(1) {
                    let count: u32 = count_match.as_str().parse().unwrap();
                    min_blue = cmp::max(min_blue, count)
                }
            }
        }

        total_sum += min_red * min_green * min_blue;
    }
    return Some(total_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
