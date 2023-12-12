aoc23_rust::solution!(12);


#[derive(Debug)]
enum SpringState {
    Unknown,
    Damaged,
    Operational,
}

fn parse_input(input: &str) -> (Vec<SpringState>, Vec<u32>) {
    let parts: Vec<&str> = input.split(' ').collect();
    let spring_states: Vec<SpringState> = parts[0].chars().map(|c| {
        match c {
            '?' => SpringState::Unknown,
            '#' => SpringState::Damaged,
            '.' => SpringState::Operational,
            _ => panic!("Invalid character"),
        }
    }).collect();
    let numbers: Vec<u32> = parts[1].split(',').map(|s| s.parse().unwrap()).collect();
    (spring_states, numbers)
}

fn count_damaged_sequences(input: &str) -> Vec<u32> {
    // for and input like ".#.###.#.######" count the contiguous # and return a vector of the counts like 1,3,1,6
    let mut counts = vec![];
    let mut count = 0;
    for c in input.chars() {
        if c == '#' {
            count += 1;
        } else {
            if count > 0 {
                counts.push(count);
                count = 0;
            }
        }
    }
    if count > 0 {
        counts.push(count);
    }
    counts
}

fn count_options(spring_states: Vec<SpringState>, numbers: Vec<u32>) -> u32 {
    // count the number of ways we can replace an unknown with a damaged sequence to produce a count_damaged_sequences matching the numbers vec
    let mut total = 1;
    let mut unknown_count = 0;
    let mut damaged_count = 0;
    let mut damaged_sequences = vec![];
    for state in spring_states {
        match state {
            SpringState::Unknown => unknown_count += 1,
            SpringState::Damaged => damaged_count += 1,
            SpringState::Operational => {
                if damaged_count > 0 {
                    damaged_sequences.push(damaged_count);
                    damaged_count = 0;
                }
            }
        }
    }
    if damaged_count > 0 {
        damaged_sequences.push(damaged_count);
    }
    println!("damaged_sequences: {:?}", damaged_sequences);
    println!("unknown_count: {:?}", unknown_count);
    println!("numbers: {:?}", numbers);
    for damaged_sequence in damaged_sequences {
        let mut count = 0;
        for number in &numbers {
            if damaged_sequence == *number {
                count += 1;
            }
        }
        total *= count;
    }
    total
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    for line in input.lines(){
        let (spring_states, numbers) = parse_input(input);
        println!("{:?}, {:?}", spring_states, numbers);
        total += count_options(spring_states, numbers);
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
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
        let result = count_damaged_sequences( ".#.###.#.######");
        assert_eq!(result, vec![1,3,1,6]);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
