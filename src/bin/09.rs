aoc23_rust::solution!(9);

pub fn part_one(input: &str) -> Option<isize> {
    let start_lists: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();

    let mut all_lines: Vec<Vec<Vec<i32>>> = vec![Vec::new(); start_lists.len()];
    for n in 0..start_lists.len() {
        all_lines[n].push(start_lists[n].clone());

        let mut last_list = all_lines[n].last().unwrap();

        while last_list.iter().any(|num| *num != 0) {
            let mut diff_list: Vec<i32> = Vec::new();
            for i in 1..last_list.len() {
                diff_list.push(last_list[i] - last_list[i - 1]);
            }
            all_lines[n].push(diff_list);
            last_list = all_lines[n].last().unwrap()
        }
        //dbg!(&all_lines[n]);
    }
    //dbg!(all_nums);

    // generate the next number
    let total: isize = all_lines
        .iter_mut()
        .map(|line| {
            let mut prev_last = 0;
            for diffs in line.iter_mut().rev() {
                prev_last += diffs.last().unwrap();
                diffs.push(prev_last);
            }
            //dbg!(&prev_last);
            prev_last as isize
        })
        .sum();

    Some(total as isize)
}

pub fn part_two(_input: &str) -> Option<isize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
