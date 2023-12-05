aoc23_rust::solution!(3);

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Part {
    num: u32,
    start_row: usize,
    start_col: usize,
    _end_row: usize,
    end_col: usize,
}
fn get_parts_from_input(input_grid: &Vec<Vec<char>>) -> Vec<Part> {
    let mut row = 0;
    let mut parts: Vec<Part> = Vec::new();
    while row < input_grid.len() {
        let mut col = 0;
        while col < input_grid[row].len() {
            if input_grid[row][col].is_numeric() {
                let mut next_num: String = "".to_string();
                let start_row = row;
                let start_col = col;
                let _end_row = row;

                while col < input_grid[row].len() && input_grid[row][col].is_numeric() {
                    next_num.push(input_grid[row][col]);
                    col += 1;
                }
                let end_col = col;

                parts.push(Part {
                    num: next_num.parse::<u32>().unwrap(),
                    start_row,
                    start_col,
                    _end_row,
                    end_col,
                });
            } else {
                col += 1;
            }
        }
        row += 1;
    }
    parts
}

fn parse_input_grid(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    // list of valid symbols
    let symbols = ['!', '@', '#', '$', '%', '^', '&', '*', '+', '-', '/', '='];

    // read the input into a 2d vec
    let input_grid: Vec<Vec<char>> = parse_input_grid(input);
    //dbg!(&input_grid);

    // find the start/end location of numbers in the input
    let parts = get_parts_from_input(&input_grid);
    //dbg!(&parts);

    let adjacent = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    // for each number, check if it has an adjacent symbol
    let total = parts
        .iter()
        .filter_map(|part| {
            let row = part.start_row;
            let mut col = part.start_col;
            let mut found_symbol = false;
            while col < part.end_col && !found_symbol {
                for &(dx, dy) in adjacent.iter() {
                    let new_row = row as i32 + dx;
                    let new_col = col as i32 + dy;
                    if new_row >= 0
                        && new_col >= 0
                        && new_row < input_grid.len() as i32
                        && new_col < input_grid[row].len() as i32
                        && symbols.contains(&input_grid[new_row as usize][new_col as usize])
                    {
                        found_symbol = true;
                        break;
                    }
                }
                col += 1;
            }
            if found_symbol {
                Some(part.num)
            } else {
                None
            }
        })
        .sum();

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    // read the input into a 2d vec
    let input_grid: Vec<Vec<char>> = parse_input_grid(input);

    // find the start/end location of numbers in the input
    let parts = get_parts_from_input(&input_grid);

    let adjacent = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut muls: HashMap<(usize, usize), Vec<Part>> = HashMap::new();
    for (row_index, row) in input_grid.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            if cell == '*' {
                muls.insert((row_index, col_index), Vec::new());
            }
        }
    }
    // for each number, check if it has an adjacent symbol
    let _valid_parts = parts
        .iter()
        .map(|part| {
            let row = part.start_row;
            let mut col = part.start_col;
            while col < part.end_col {
                for &(dx, dy) in adjacent.iter() {
                    let new_row = row.saturating_add_signed(dx);
                    let new_col = col.saturating_add_signed(dy);
                    if new_row < input_grid.len()
                        && new_col < input_grid[row].len()
                        && input_grid[new_row][new_col] == '*'
                    {
                        let mul = muls.get_mut(&(new_row, new_col)).expect("found new *");
                        if !mul.contains(part) {
                            mul.push(*part);
                            break;
                        }
                    }
                }
                col += 1;
            }
        })
        .count();

    //dbg!(&muls);
    let total = muls
        .iter()
        .map(|((_row, _col), parts)| {
            if parts.len() == 2 {
                parts[0].num * parts[1].num
            } else {
                0
            }
        })
        .sum();
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
