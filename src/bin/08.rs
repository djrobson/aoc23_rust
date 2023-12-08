aoc23_rust::solution!(8);
use std::collections::HashMap;

fn get_next_direction(directions: &str, index: usize) -> char {
    let wrap_index = index % directions.len();
    directions.chars().nth(wrap_index).unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut graph: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut lines = input.lines();
    let directions = lines.next().unwrap();
    lines.next(); // skip a ;line

    lines
        .map(|line| {
            let root = &line[0..3];
            let left = &line[7..10];
            let right = &line[12..15];

            graph.insert(root, (left, right));
        })
        .count();

    let mut current_location = "AAA";
    let mut step_count = 0;
    while current_location != "ZZZ" {
        let next_direction = get_next_direction(directions, step_count);
        match next_direction {
            'L' => current_location = graph.get(current_location).unwrap().0,
            'R' => current_location = graph.get(current_location).unwrap().1,
            _ => panic!("unexpected direction"),
        }
        step_count += 1;
    }
    Some(step_count as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
