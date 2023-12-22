aoc23_rust::solution!(22);

pub fn parse_input(input: &str) -> Vec<((i32, i32, i32), (i32, i32, i32))> {
    let mut result = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split('~').collect();
        if parts.len() != 2 {
            panic!("Invalid input ~");
        }

        let parse_tuple = |s: &str| -> Option<(i32, i32, i32)> {
            let nums: Vec<i32> = s.split(',').filter_map(|x| x.parse().ok()).collect();
            if nums.len() == 3 {
                Some((nums[0], nums[1], nums[2]))
            } else {
                None
            }
        };

        if let (Some(t1), Some(t2)) = (parse_tuple(parts[0]), parse_tuple(parts[1])) {
            result.push((t1, t2));
        } else {
            panic!("Invalid input i32s");
        }
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {

    let blocks = parse_input(input);

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
