aoc23_rust::solution!(4);

fn parse_input(input: &str) -> Vec<(Vec<u32>, Vec<u32>)> {
    input
        .lines()
        .map(|line| {
            let nums: Vec<&str> = line.split(':').collect::<Vec<&str>>()[1]
                .split('|')
                .collect();
            let winning_nums = nums[0]
                .trim()
                .split(' ')
                .filter_map(|x| x.parse().ok())
                .collect();
            let have_nums = nums[1]
                .trim()
                .split(' ')
                .filter_map(|x| x.parse().ok())
                .collect();
            (winning_nums, have_nums)
        })
        .collect()
}
pub fn part_one(input: &str) -> Option<u32> {
    let num_lists: Vec<(Vec<u32>, Vec<u32>)> = parse_input(input);

    let counts: Vec<usize> = num_lists
        .iter()
        .map(|win_have| {
            win_have
                .0
                .iter()
                .filter(|num| win_have.1.contains(*num))
                .count()
        })
        .collect();

    let total: u32 = counts
        .iter()
        .map(|count| {
            if count >= &1 {
                u32::pow(2, *count as u32 - 1)
            } else {
                0
            }
        })
        .sum();
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let num_lists: Vec<(Vec<u32>, Vec<u32>)> = parse_input(input);

    let my_win_counts: Vec<usize> = num_lists
        .iter()
        .map(|win_have| {
            win_have
                .0
                .iter()
                .filter(|num| win_have.1.contains(*num))
                .count()
        })
        .collect();

    // initialize the list backwards since we're guaranteed to terminate
    let mut copies: Vec<usize> = vec![0; num_lists.len()];
    for x in (0..num_lists.len()).rev() {
        copies[x] = if my_win_counts[x] > 0 {
            // grab items from vec at offset x+1 to x+my_win_counts[x]
            // and sum them together
            let list_after = copies.split_at(x + 1).1;
            let list_until = list_after.split_at(my_win_counts[x]).0;
            list_until.iter().sum::<usize>() + 1
        } else {
            1
        };
    }

    Some(copies.iter().sum::<usize>() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
