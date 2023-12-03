aoc23_rust::solution!(3);

#[derive(Debug,Clone,Copy)]
struct Part {
    num: u32,
    start_row: usize,
    start_col: usize,
    end_row: usize,
    end_col: usize,
}

pub fn part_one(input: &str) -> Option<u32> {
    // list of valid symbols
    let symbols = ['!', '@', '#', '$', '%', '^', '&', '*', '+', '-','/','='];

    // read the input into a 2d vec
    let input_grid: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    //dbg!(&input_grid);

    // find the start/end location of numbers in the input
    let mut row = 0;
    let mut parts: Vec<Part> = Vec::new();
    while row < input_grid.len() {
        let mut col = 0;
        while col < input_grid[row].len() {
            if input_grid[row][col].is_numeric() {
                let mut next_num: String = "".to_string();
                let start_row = row;
                let start_col = col;
                let end_row = row;

                while col < input_grid[row].len() && input_grid[row][col].is_numeric() {
                    next_num.push(input_grid[row][col]);
                    col += 1;
                }
                let end_col = col;

                parts.push(Part {
                    num: next_num.parse::<u32>().unwrap(),
                    start_row: start_row,
                    start_col: start_col,
                    end_row: end_row,
                    end_col: end_col,
                });
            } else {
                col +=1;
            }
        }
        row += 1;
    }
    //dbg!(&parts);

    let adjascent = [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)];
    // for each number, check if it has an adjascent symbol
    let total = parts.iter().filter_map(|part| {
        let row = part.start_row;
        let mut col = part.start_col;
        let mut found_symbol = false;
        while col < part.end_col && found_symbol == false {
            for &(dx, dy) in adjascent.iter() {
                let new_row = row as i32 + dx;
                let new_col = col as i32 + dy;
                if new_row >= 0 && new_col >= 0 && new_row < input_grid.len() as i32 && new_col < input_grid[row].len() as i32 {
                    if symbols.contains(&input_grid[new_row as usize][new_col as usize]) {
                        found_symbol = true;
                        break;
                    }
                }
            }
            col += 1;
        }
        if found_symbol {
            Some(part.num)
        } else {
            None
        }
    }).sum();

    Some(total)
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
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
