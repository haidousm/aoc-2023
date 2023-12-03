use std::collections::HashMap;

use regex::Regex;

advent_of_code::solution!(1);
pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut total_sum = 0;
    for line in lines {
        let chars = line.chars();
        let mut left_ptr = 0;
        let mut right_ptr = chars.count() - 1;

        let mut left_char: Option<u32> = None;
        let mut right_char: Option<u32> = None;
        while left_ptr <= right_ptr && (left_char.is_none() || right_char.is_none()) {
            let left = line.chars().nth(left_ptr).unwrap();
            let right = line.chars().nth(right_ptr).unwrap();

            if left_char.is_none() {
                if left.is_digit(10) {
                    left_char = Some(left.to_digit(10).unwrap());
                } else {
                    left_ptr += 1;
                }
            }

            if right_char.is_none() {
                if right.is_digit(10) {
                    right_char = Some(right.to_digit(10).unwrap());
                } else {
                    right_ptr -= 1;
                }
            }
        }
        total_sum += (left_char.unwrap() * 10) + right_char.unwrap();
    }
    return Some(total_sum);
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut total_sum = 0;
    let regexp_1: Regex = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine).*(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let regexp_2: Regex = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let map: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    for line in lines {
        let mut left_num: Option<u32> = None;
        let mut right_num: Option<u32> = None;
        if let Some(matches) = regexp_1.captures(line) {
            let maybe_first_match = matches.get(1);
            let maybe_second_match = matches.get(2);
            if let (Some(first_match), Some(second_match)) = (maybe_first_match, maybe_second_match)
            {
                let first_match_str = first_match.as_str();
                let second_match_str = second_match.as_str();
                if map.contains_key(first_match_str) {
                    left_num = map.get(first_match_str).copied();
                } else {
                    left_num = first_match_str.chars().nth(0).unwrap().to_digit(10);
                }
                if map.contains_key(second_match_str) {
                    right_num = map.get(second_match_str).copied();
                } else {
                    right_num = second_match_str.chars().nth(0).unwrap().to_digit(10);
                }
            } else if let Some(first_match) = maybe_first_match {
                let first_match_str = first_match.as_str();
                if map.contains_key(first_match_str) {
                    left_num = map.get(first_match_str).copied();
                    right_num = left_num;
                } else {
                    left_num = first_match_str.chars().nth(0).unwrap().to_digit(10);
                    right_num = left_num;
                }
            } else if let Some(second_match) = maybe_second_match {
                let second_match_str = second_match.as_str();
                if map.contains_key(second_match_str) {
                    right_num = map.get(second_match_str).copied();
                    left_num = right_num;
                } else {
                    right_num = second_match_str.chars().nth(0).unwrap().to_digit(10);
                    left_num = right_num;
                }
            }
            total_sum += (left_num.unwrap() * 10) + right_num.unwrap();
        } else if let Some(other_matches) = regexp_2.captures(line) {
            if let Some(first_match) = other_matches.get(1) {
                let first_match_str = first_match.as_str();
                if map.contains_key(first_match_str) {
                    left_num = map.get(first_match_str).copied();
                    right_num = left_num;
                } else {
                    left_num = first_match_str.chars().nth(0).unwrap().to_digit(10);
                    right_num = left_num;
                }
                total_sum += (left_num.unwrap() * 10) + right_num.unwrap();
            }
        }
    }
    return Some(total_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
