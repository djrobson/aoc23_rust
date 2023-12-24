use memoize::memoize;
use rayon::prelude::*;

aoc23_rust::solution!(12);

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

#[memoize]
fn count_options(spring_states: Vec<u8>, numbers: Vec<u8>) -> usize {
    match (spring_states.is_empty(), numbers.is_empty()) {
        (true, true) => return 0,
        (true, false) => return 0,
        (false, true) => return 0,
        _ => (),
    }

    if spring_states[0] == b'.' {
        // remove the . and try again
        let mut new_spring_states = spring_states.clone();
        new_spring_states.remove(0);
        let count = count_options(new_spring_states, numbers);
        return count;
    }

    // count the number of ways we can replace an Unknown with a Damaged or Operatioanl to produce a count_damaged_sequences matching the numbers vec
    let mut ways = 0;

    for i in 0..spring_states.len() {
        // scan past all the #
        if spring_states[i] == b'?' {
            // try it with a #
            if numbers.len() == 1
                && i == numbers[0] as usize
                && spring_states[i..].iter().all(|c| *c != b'#')
            {
                // end  the last sequence and jump to the end
                println!("match 1 {:?} {:?}", spring_states, numbers);
                return 1;
            }
            if i < numbers[0] as usize {
                let mut oper_spring_states = spring_states.clone(); // Create a mutable copy of spring_states
                oper_spring_states[i] = b'#';

                let count = count_options(oper_spring_states, numbers.clone());
                ways += count;
            }

            // try it with a .
            let mut broken_spring_states = spring_states.to_owned(); // Create another mutable copy of spring_states
            broken_spring_states[i] = b'.';

            let prefix = count_damaged_sequences(&broken_spring_states);
            if !prefix.is_empty() && numbers.clone().starts_with(&prefix) {
                let spring_suffix = broken_spring_states
                    .split_at(numbers[0] as usize + 1)
                    .1
                    .to_vec();
                let mut numbers_suffix = numbers.clone();
                numbers_suffix.remove(0);
                let count = count_options(spring_suffix, numbers_suffix);
                ways += count;
            } else {
                ways += count_options(broken_spring_states, numbers.clone());
            }

            return ways;
        }
    }
    // we found no ? so check if we're good
    if count_damaged_sequences(&spring_states) == numbers
        && !spring_states.iter().any(|c| *c == b'?')
    {
        println!("match 2 {} {:?} {:?}", ways, spring_states, numbers);
        ways += 1;
    }

    ways
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut total = 0;
    for line in input.lines() {
        let mut parts = line.split(' ');
        let spring_states: Vec<u8> = parts.next().unwrap().as_bytes().to_vec();
        let numbers: Vec<u8> = parts
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        total += count_options(spring_states, numbers);
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

            let count = count_options(spring_states.as_bytes().to_vec(), repeat_numbers);
            println!("finished {} with count: {}", line, count,);
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

    #[ignore]
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

    #[ignore]
    #[test]
    fn test_part_one_5() {
        let result = part_one("????.######..#####. 1,6,5");
        assert_eq!(result, Some(4));
    }

    #[ignore]
    #[test]
    fn test_part_one_5_2() {
        let result = part_one("??.#. 1,1");
        assert_eq!(result, Some(2));
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
    fn test_part_one_8() {
        let result = part_one("### 3");
        assert_eq!(result, Some(1));
    }
    #[test]
    fn test_part_one_9() {
        let result = part_one(".###. 3");
        assert_eq!(result, Some(1));
    }
    #[test]
    fn test_part_one_10() {
        let result = part_one(".?#?. 2");
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_11() {
        let result = part_one("?#? 2");
        assert_eq!(result, Some(2));
    }

    #[ignore]
    #[test]
    fn test_part_two() {
        let result = part_two("????.######..#####. 1,6,5");
        assert_eq!(result, Some(2500));
    }
}
