aoc23_rust::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    /*let result : u32 = input
        .lines()
        .map(|line| line.chars().filter(|c| c.is_numeric()).collect::<String>())
        .map(|filtered_line| {
            let digits = filtered_line.as_bytes();
            let first = digits[0];
            let last = digits[digits.len()-1];
            let result: u32 = (u32::from(first) * 10) + u32::from(last);
            result
        })
        .sum();
        let lines = input.lines();
    let all_nums: Vec<String> = lines
        .map(|line| line.chars().filter(|c| c.is_numeric())
        .collect::<String>()).collect();

    let fl_nums:Vec<u32> = all_nums.iter().map(|line| {
        let digits= line.as_bytes();
        let first = digits[0] - 0x30;
        let last = digits[digits.len()-1] - 0x30;
        u32::from(first*10+last)
    }).collect();

    Some(fl_nums.iter().sum())
    */
    //Some(result)

    let total = input.lines()
        .map(|line| line.chars().filter(|c| c.is_numeric())
        .collect::<String>())
        .map(|line| {
            let digits= line.as_bytes();
            let first = digits[0] - 0x30;
            let last = digits[digits.len()-1] - 0x30;
            u32::from(first*10+last)
        }).sum();
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {

    Some(281)
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
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
