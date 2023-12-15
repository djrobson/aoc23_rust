aoc23_rust::solution!(12);

#[derive(Debug, Clone)]
enum SpringState {
    Unknown,
    Damaged,
    Operational,
}

fn parse_input(input: &str) -> (Vec<SpringState>, Vec<u32>) {
    let parts: Vec<&str> = input.split(' ').collect();
    let spring_states: Vec<SpringState> = parts[0]
        .chars()
        .map(|c| match c {
            '?' => SpringState::Unknown,
            '#' => SpringState::Damaged,
            '.' => SpringState::Operational,
            _ => panic!("Invalid character"),
        })
        .collect();
    let numbers: Vec<u32> = parts[1].split(',').map(|s| s.parse().unwrap()).collect();
    (spring_states, numbers)
}

fn count_damaged_sequences(input: &str) -> Vec<u32> {
    // for and input like ".#.###.#.######" count the contiguous # and return a vector of the counts like 1,3,1,6
    let mut counts = vec![];
    let mut count = 0;
    for c in input.chars() {
        match c {
            '#' => count += 1,
            '.' => {
                if count > 0 {
                    counts.push(count);
                    count = 0;
                }
            }
            '?' => panic!("unexpected unknown"),
            _ => panic!("unexpected character"),
        }
    }
    if count > 0 {
        counts.push(count);
    }
    counts
}

fn count_options(spring_states: Vec<SpringState>, numbers: Vec<u32>) -> u32 {
    // count the number of ways we can replace an unknown with a damaged sequence to produce a count_damaged_sequences matching the numbers vec
    let mut ways = 0;
    // recurse if there's more than 1 unknownt
    for i in 0..spring_states.len() {
        if let SpringState::Unknown = spring_states[i] {
            let mut temp_states = spring_states.clone();
            temp_states[i] = SpringState::Damaged;
            let temp_counts = count_damaged_sequences(
                &temp_states
                    .iter()
                    .map(|s| match s {
                        SpringState::Unknown => '?',
                        SpringState::Damaged => '#',
                        SpringState::Operational => '.',
                    })
                    .collect::<String>(),
            );
            if temp_counts == numbers {
                ways += 1;
            }
            temp_states[i] = SpringState::Operational;
            let temp_counts = count_damaged_sequences(
                &temp_states
                    .iter()
                    .map(|s| match s {
                        SpringState::Unknown => '?',
                        SpringState::Damaged => '#',
                        SpringState::Operational => '.',
                    })
                    .collect::<String>(),
            );
            if temp_counts == numbers {
                ways += 1;
            }
        }
    }
    ways
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    for line in input.lines() {
        let (spring_states, numbers) = parse_input(line);
        println!("{:?}, {:?}", spring_states, numbers);
        total += count_options(spring_states, numbers);
    }

    Some(total)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
/*
    #[test]
    fn test_part_one_1() {
        let result = part_one("???.### 1,1,3");
        assert_eq!(result, Some(1));
    }
    #[test]
    fn test_part_one_2() {
        let result = part_one(".??..??...?##. 1,1,3");
        assert_eq!(result, Some(4));
    }
    #[test]
    fn test_part_one_3() {
        let result = part_one("?#?#?#?#?#?#?#? 1,3,1,6");
        assert_eq!(result, Some(4));
    }
    #[test]
    fn test_part_one_4() {
        let result = part_one("????.#...#... 4,1,1");
        assert_eq!(result, Some(1));
    }
    #[test]
    fn test_part_one_5() {
        let result = part_one("????.######..#####. 1,6,5");
        assert_eq!(result, Some(4));
    }
    #[test]
    fn test_part_one_6() {
        let result = part_one("?###???????? 3,2,1");
        assert_eq!(result, Some(10));
    }
*/
    #[test]
    fn test_part_one_7() {
        let result = count_damaged_sequences(".#.###.#.######");
        assert_eq!(result, vec![1, 3, 1, 6]);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
