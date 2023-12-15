aoc23_rust::solution!(15);

fn hash1(input: &str) -> u8 {
    input.as_bytes().iter().fold(0, |hash, ch| {
        (((hash as u32 + *ch as u32) * 17) & 0xff) as u8
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .trim_end()
            .split(',')
            .map(|record| hash1(record) as u32)
            .sum(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("rn=1", 30)]
    #[case("cm-", 253)]
    #[case("qp=3", 97)]
    #[case("cm=2", 47)]
    #[case("qp-", 14)]
    #[case("pc=4", 180)]
    #[case("ot=9", 9)]
    #[case("ab=5", 197)]
    #[case("pc-", 48)]
    #[case("pc=6", 214)]
    #[case("ot=7", 231)]
    fn test_hash1(#[case] input: &str, #[case] expected: u8) {
        let result = hash1(input);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_part_one() {
        let result = part_one(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
