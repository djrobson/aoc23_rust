aoc23_rust::solution!(2);
use std::collections::HashMap;


fn parse_input(input: &str) -> Vec<HashMap<&str, u32>> {
    input
        .trim()
        .split('\n')
        .map(|line| {
            let mut game: HashMap<&str, u32> = HashMap::new();
            let parts: Vec<&str> = line.split(": ").collect();
            let colors: Vec<&str> = parts[1].split("; ").collect();
            for color in colors {
                let color_counts: Vec<&str> = color.split(", ").collect();
                for color_count in color_counts {
                    let color_count_pair: Vec<&str> = color_count.split(" ").collect();
                    let new_count: u32 = color_count_pair[0].parse().unwrap();
                    match game.get(color_count_pair[1]) {
                        Some(x) => {
                            if x < &new_count {
                                game.insert(color_count_pair[1], new_count);
                            }
                        }
                        None => {
                            game.insert(color_count_pair[1], new_count);
                        }
                    };
                }
            }
            game
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let games = parse_input(input);
    
    // look for games with only 12 red cubes, 13 green cubes, and 14 blue cubes
    let total = games
        .iter()
        .enumerate()
        .map(|(game_num, colors)| {
            if colors.get("red").is_some_and(|cnt| cnt <= &12)
                && colors.get("green").is_some_and(|cnt| cnt <= &13)
                && colors.get("blue").is_some_and(|cnt| cnt <= &14)
            {
                game_num + 1 // enumerate counts from 0
            } else {
                0
            }
        })
        .sum();
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = parse_input(input);

    let total = games
        .iter()
        .map(|colors| {
            let red = colors.get("red").unwrap_or(&0);
            let blue = colors.get("blue").unwrap_or(&0);
            let green = colors.get("green").unwrap_or(&0);
            red*blue*green // multiple the minum values together
        })
        .sum(); // return the sum of the values
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
