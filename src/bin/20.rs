aoc23_rust::solution!(20);

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one("broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a");
        assert_eq!(result, Some(32_000_000));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one("broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output");
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, Some(11687500));
    }
}
