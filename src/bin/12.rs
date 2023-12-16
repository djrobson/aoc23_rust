aoc23_rust::solution!(12);

/*#[derive(Debug, Clone, Eq, PartialEq)]
enum SpringState {
    Unknown,
    Damaged,
    Operational,
}*/

/*fn parse_input(input: &str) -> (Vec<SpringState>, Vec<u32>) {
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
}*/

fn count_damaged_sequences(input: &[u8]) -> Vec<u32> {
    // for and input like ".#.###.#.######" count the contiguous # and return a vector of the counts like 1,3,1,6
    let mut counts = vec![];
    let mut count = 0;
    for c in input.iter() {
        match c {
            b'#' => count += 1,
            b'.' => {
                if count > 0 {
                    counts.push(count);
                    count = 0;
                }
            }
            b'?' => panic!("unexpected unknown"),
            _ => panic!("unexpected character"),
        }
    }
    if count > 0 {
        counts.push(count);
    }
    counts
}

fn count_options(spring_states: &[u8], numbers: &Vec<u32>) -> u32 {
    // count the number of ways we can replace an Unknown with a Damaged or Operatioanl to produce a count_damaged_sequences matching the numbers vec
    let mut ways = 0;
    // recurse if there's more than 1 unknown
    let unknown_count = spring_states.iter().filter(|s| **s == b'?').count();
    if unknown_count > 0 {
        for i in 0..spring_states.len() {
            if spring_states[i] == b'?' {
                let mut new_spring_states = spring_states.to_owned(); // Create a mutable copy of spring_states
                new_spring_states[i] = b'#';
                ways += count_options(&new_spring_states, numbers); // Pass the mutable copy to count_options
                let mut new_spring_states = spring_states.to_owned(); // Create another mutable copy of spring_states
                new_spring_states[i] = b'.';
                ways += count_options(&new_spring_states, numbers); // Pass the mutable copy to count_options
                break;
            }
        }
    } else {
        // we have 1 unknown, so we can just try all the options
        let counts = count_damaged_sequences(spring_states);
        if &counts == numbers {
            ways += 1;
        }
    }
    ways
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    for line in input.lines() {
        let mut parts = line.split(' ');
        let spring_states = parts.next().unwrap().as_bytes();
        let numbers = parts
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        //println!("{:?}, {:?}", spring_states, &numbers);
        total += count_options(spring_states, &numbers);
    }

    Some(total)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_part_one_7() {
        let result = count_damaged_sequences(b".#.###.#.######");
        assert_eq!(result, vec![1, 3, 1, 6]);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
