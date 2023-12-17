use rayon::prelude::*;
use std::collections::HashMap;

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

fn count_damaged_sequences(input: &[u8]) -> Vec<u8> {
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
            b'?' => return counts, // exist early if we hit an unknown
            _ => panic!("unexpected character"),
        }
    }
    if count > 0 {
        counts.push(count);
    }
    counts
}

fn count_options(
    spring_states: &[u8],
    numbers: &Vec<u8>,
    total_working_needed: u8,
    total_working_found: u8,
    unknown_count: u8,
    //seen: &mut HashMap<&[u8], usize>,
) -> usize {
    // count the number of ways we can replace an Unknown with a Damaged or Operatioanl to produce a count_damaged_sequences matching the numbers vec
    if total_working_found > total_working_needed
        || total_working_needed > (total_working_found + unknown_count)
    {
        return 0;
    }

    if unknown_count == 0 {
        // no more unknowns, check if we have a match
        let prefix = count_damaged_sequences(spring_states);
        if &prefix == numbers {
            return 1;
        } else {
            return 0;
        }
    }

    let mut ways = 0;

    for i in 0..spring_states.len() {
        if spring_states[i] == b'?' {
            // try it with a #
            let mut new_spring_states = spring_states.to_owned(); // Create a mutable copy of spring_states
            new_spring_states[i] = b'#';
            // grab all the u8 from the end of new_spring_states that are not ?
            //let prefix = count_damaged_sequences(&new_spring_states);

            let prefix = count_damaged_sequences(&new_spring_states);
            if numbers.starts_with(&prefix) {
                let mut total_prefix_found = prefix.iter().sum::<u8>();

                // memoize all content after the first total_prefix_found #s and numbers after prefix.len()
                /*let spring_suffix = new_spring_states
                            .iter()
                            .skip_while(|c| {
                                if total_prefix_found > 0 {
                                    if *c == &b'#' {
                                        total_prefix_found -= 1;
                                    }
                                    true
                                } else {
                                    false
                                }
                            }).collect();

                let numbers_suffix = numbers
                            .iter()
                            .skip(prefix.len())
                            .collect();
                // check see hashtable for spring_suffic, numbers_suffix tuple
                let suffix = (spring_suffix, numbers_suffix);
                if seen.contains_key(&suffix) {
                    ways += seen.get(&suffix).unwrap();
                    break;
                }*/

                // check if the next stretch of working springs is too long
                match numbers.get(prefix.len()) {
                    Some(n) => {
                        let next_spring_count = new_spring_states
                            .iter()
                            .skip_while(|c| {
                                if total_prefix_found > 0 {
                                    if *c == &b'#' {
                                        total_prefix_found -= 1;
                                    }
                                    true
                                } else {
                                    false
                                }
                            })
                            .skip_while(|c| *c == &b'.')
                            .take_while(|c| *c == &b'#')
                            .count();
                        if next_spring_count <= *n as usize {
                            let count = count_options(
                                &new_spring_states,
                                numbers,
                                total_working_needed,
                                total_working_found + 1,
                                unknown_count - 1,
                                //seen,
                            );
                            //seen.insert(suffix, count);
                            ways += count;
                        }
                    }
                    None => {
                        // we matched the whole string, it doens't matter what we do with the rest of the unknowns
                        ways += 1;
                        break;
                    }
                }
            }

            // try it with a .
            let mut new_spring_states = spring_states.to_owned(); // Create another mutable copy of spring_states
            new_spring_states[i] = b'.';

            let suffix = new_spring_states.split(|c| *c == b'?').last();
            //if seen.contains_key(&suffix) {
            //    ways += seen.get(&suffix).unwrap();
            //} else {

            let prefix = count_damaged_sequences(&new_spring_states);
            if numbers.starts_with(&prefix) {
                let count = count_options(
                    &new_spring_states,
                    numbers,
                    total_working_needed,
                    total_working_found,
                    unknown_count - 1,
                    //seen,
                );
                //seen.insert(suffix.clone(), count);
                ways += count;
            }
            //}
            break;
        }
    }

    ways
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut total = 0;
    for line in input.lines() {
        let mut parts = line.split(' ');
        let spring_states = parts.next().unwrap().as_bytes();
        let numbers: Vec<u8> = parts
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        //println!("{:?}, {:?}", spring_states, &numbers);
        let total_working_needed: u8 = numbers.iter().sum();
        let total_working_found = spring_states.iter().filter(|s| **s == b'#').count();
        let unknown_count = spring_states.iter().filter(|s| **s == b'?').count();

        let mut seen: HashMap<&[u8], usize> = HashMap::new();
        total += count_options(
            spring_states,
            &numbers,
            total_working_needed,
            total_working_found as u8,
            unknown_count as u8,
            //&mut seen,
        );
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<usize> {
    let lines: Vec<&str> = input.lines().collect();
    let total: usize = lines
        .par_iter()
        .map(|line| {
            let mut parts = line.split(' ');
            let spring_string = parts.next().unwrap();
            let spring_states = format!(
                "{}?{}?{}?{}?{}",
                spring_string, spring_string, spring_string, spring_string, spring_string
            );
            let numbers: Vec<u8> = parts
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            let repeat_numbers = numbers.repeat(5);
            //println!("{:?}, {:?}", spring_states, &numbers);

            let total_working_needed: u8 = repeat_numbers.iter().sum();
            let total_working_found = spring_states
                .as_bytes()
                .iter()
                .filter(|s| **s == b'#')
                .count();
            let unknown_count = spring_states
                .as_bytes()
                .iter()
                .filter(|s| **s == b'?')
                .count();

            let mut seen: HashMap<&[u8], usize> = HashMap::new();
            let count = count_options(
                spring_states.as_bytes(),
                &repeat_numbers,
                total_working_needed,
                total_working_found as u8,
                unknown_count as u8,
                //&mut seen,
            );
            println!(
                "finished {} with count: {}, cache records {}",
                line,
                count,
                seen.len()
            );
            count as usize
        })
        .sum();

    println!("total: {}", total);

    Some(total)
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
        assert_eq!(result, Some(1));
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
        let result = part_two("????.######..#####. 1,6,5");
        assert_eq!(result, Some(2500));
    }
}
