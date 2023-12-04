use std::collections::VecDeque;

aoc23_rust::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let num_lists: Vec<(Vec<u32>, Vec<u32>)> = input
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
        .collect();

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
    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let num_lists: Vec<(Vec<u32>, Vec<u32>)> = input
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
        .collect();

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

    // TODO: re-write this with dynamic programming
    // initialize the list with the starting cards
    let mut copies: VecDeque<usize> = VecDeque::new();
    for x in 0..num_lists.len() {
        copies.push_back(x); // we default to the list befor we add the copies
    }

    // iterate through a growing list of copies
    let mut index = 0;
    while index < copies.len() {
        let which_card = copies[index];
        let cnt = my_win_counts[which_card];
        //insert all the new copies on the end
        for next in 0..cnt {
            copies.push_back(which_card + next + 1);
        }
        index += 1;
    }

    Some(copies.len() as u32)
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
