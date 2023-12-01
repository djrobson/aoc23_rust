aoc23_rust::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let total = input
        .lines()
        .map(|line| line.chars().filter(|c| c.is_numeric()).collect::<String>())
        .map(|line| {
            let digits = line.as_bytes();
            let first = digits[0] - 0x30;
            let last = digits[digits.len() - 1] - 0x30;
            u32::from(first * 10 + last)
        })
        .sum();
    Some(total)
}

fn is_ascii_numeric(input: u8) -> bool {
    input >= b'1' && input <= b'9'
}
pub fn part_two(input: &str) -> Option<u32> {
    let patterns = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();

    // convert 'one' 'two' 'three' to 1 2 3
    let all_nums: Vec<Vec<u8>> = lines
        .iter()
        //.take(30)
        .map(|line| {
            let mut nums: Vec<u8> = Vec::new();
            let mut offset = 0;
            'location: while offset < line.len() {
                if is_ascii_numeric(line.as_bytes()[offset]) {
                    nums.push(line.as_bytes()[offset] - b'0');
                    offset += 1;
                    continue 'location;
                }
                for pat in patterns {
                    if line[offset..].starts_with(pat.0) {
                        nums.push(pat.1);
                        // overlapping substrings are allowed in the inputs
                        // offset += pat.0.len();
                        offset += 1;
                        continue 'location;
                    }
                }
                offset += 1;
            }
            let num = {
                let first = nums[0];
                let last = nums[nums.len() - 1];
                u32::from(first * 10 + last)
            };
            //println!("{} became {:?} with val {}", line, &nums, num);
            nums
        })
        .collect();

    let fl_nums = all_nums.iter().map(|line| {
        let first = line[0];
        let last = line[line.len() - 1];
        u32::from(first * 10 + last)
    });
    Some(fl_nums.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_second_example("examples", DAY));
        assert_eq!(result, Some(281));
    }
    #[test]
    fn test_part_two_real() {
        let result = part_two(&aoc23_rust::template::read_file("inputs", DAY));
        assert_eq!(result, Some(281));
    }
}
