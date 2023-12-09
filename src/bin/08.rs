aoc23_rust::solution!(8);
use std::collections::HashMap;

fn is_ending_location(location: &str) -> bool {
    location.ends_with('Z')
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut graph: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut lines = input.lines();
    let mut directions = lines.next().unwrap().chars().cycle();
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
        let next_direction = directions.next().unwrap();
        match next_direction {
            'L' => current_location = graph.get(current_location).unwrap().0,
            'R' => current_location = graph.get(current_location).unwrap().1,
            _ => panic!("unexpected direction"),
        }
        step_count += 1;
    }
    Some(step_count as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut graph: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut lines = input.lines();
    let mut directions = lines.next().unwrap().chars().cycle();
    lines.next(); // skip a ;line

    lines
        .map(|line| {
            let root = &line[0..3];
            let left = &line[7..10];
            let right = &line[12..15];

            graph.insert(root, (left, right));
        })
        .count();

    let starting_locations: Vec<&str> = graph
        .keys()
        .copied()
        .filter(|&key| key.ends_with('A'))
        .collect();
    let mut end_cycles: Vec<Option<u32>> = vec![None; starting_locations.len()];
    let mut current_locations: Vec<&str> = starting_locations;
    let mut step_count = 0;

    while end_cycles.iter().any(|x| x.is_none()) {
        let next_direction = directions.next().unwrap();
        step_count += 1;

        for n in 0..current_locations.len() {
            if end_cycles[n].is_some() {
                continue;
            }
            let location = current_locations[n];
            let next_location = match next_direction {
                'L' => graph.get(location).unwrap().0,
                'R' => graph.get(location).unwrap().1,
                _ => panic!("unexpected direction"),
            };
            current_locations[n] = next_location;
            if is_ending_location(next_location) {
                end_cycles[n] = Some(step_count);
            }
        }
    }

    lcmx::lcmx(
        &end_cycles
            .iter()
            .map(|num| num.unwrap() as u64)
            .collect::<Vec<u64>>(),
    )
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
        let result = part_two(&aoc23_rust::template::read_second_example("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
