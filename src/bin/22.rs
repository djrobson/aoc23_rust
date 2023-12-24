aoc23_rust::solution!(22);
use glam::IVec3;

pub fn parse_input(input: &str) -> Vec<(IVec3, IVec3)> {
    let mut result = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split('~').collect();
        if parts.len() != 2 {
            panic!("Invalid input ~");
        }

        let parse_tuple = |s: &str| -> Option<IVec3> {
            let nums: Vec<i32> = s.split(',').filter_map(|x| x.parse().ok()).collect();
            if nums.len() == 3 {
                Some(IVec3::new(nums[0], nums[1], nums[2]))
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
    let _blocks = parse_input(input);
    // record each shape in a 3d space
    // let each shape fall until it hits the ground or a lower shape

    None
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
