aoc23_rust::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let input_lines: Vec<&str> = input.lines().collect();
    let time_list: Vec<u32> = input_lines[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    let distance_list: Vec<u32> = input_lines[1]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    let race_details: Vec<(&u32, &u32)> = time_list.iter().zip(distance_list.iter()).collect();

    let score: usize = race_details
        .iter()
        .map(|&race| {
            (0..*race.0)
                .filter(|&wait_secs| (*race.0 - wait_secs) * wait_secs > *race.1)
                .count()
        })
        .product();
    Some(score as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    // this would have been way faster to just type the two numbers
    let input_lines: Vec<&str> = input.lines().collect();
    let time: usize = input_lines[0]
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();
    let distance: usize = input_lines[1]
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();

    /* TODO: consider using a binary search across the range to find:
    the lowest winning number where n-1 fails and n succeeds
    the highest winning number where n+1 fails and n succeeds */
    let score: usize = (0..time)
        .filter(|&wait_secs| (time - wait_secs) * wait_secs > distance)
        .count();
    Some(score as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
