aoc23_rust::solution!(13);

fn parse_input(input: &str) -> Vec<Vec<Vec<u8>>> {
    // find 2d grids split by a blank line
    // collect each grid into a 2d vec
    // collect all the grids into a vec
    let mut grids = Vec::new();
    for grid_input in input.split("\n\n") {
        let grid = grid_input
            .lines()
            .map(|line| line.bytes().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        grids.push(grid);
    }
    grids
}

#[derive(PartialEq, Eq)]
enum Symmetry {
    Horizontal,
    Vertical,
}
use Symmetry::*;

fn print_grid(grid: &Vec<Vec<u8>>) {
    for row in grid {
        for col in row {
            print!("{}", *col as char);
        }
        println!();
    }
}

fn find_reflection(grid: &Vec<Vec<u8>>) -> (Symmetry, usize) {
    // for each column in grid, check for horizontal symmetry
    // for each row in grid, check for vertical symmetry
    // return the direction and index of the symmetry

    for i in 1..grid.len() {
        // check for each row before i is equal to each reflected row after i
        // ignore a row if the other side is missing
        let top = grid[0..i]
            .iter()
            .rev()
            .map(|row| row.to_vec())
            .collect::<Vec<_>>();
        let bottom = grid[i..].to_vec();

        if top.len() < bottom.len() {
            if bottom.starts_with(&top) {
                return (Vertical, i);
            }
        } else {
            if top.starts_with(&bottom) {
                return (Vertical, i);
            }
        }
    }

    for i in 1..grid[0].len() {
        // grab all columns before i and reverse them, store result in left
        // grab all columns after i, store result in right
        // check if left is equal to right
        let mut left: Vec<Vec<u8>> = Vec::new();
        for col in (0..i).rev() {
            let mut col_vec = Vec::new();
            for row in 0..grid.len() {
                col_vec.push(grid[row][col]);
            }
            left.push(col_vec);
        }
        let mut right: Vec<Vec<u8>> = Vec::new();
        for col in i..grid[0].len() {
            let mut col_vec = Vec::new();
            for row in 0..grid.len() {
                col_vec.push(grid[row][col]);
            }
            right.push(col_vec);
        }

        if left.len() < right.len() {
            if right.starts_with(&left) {
                return (Horizontal, i);
            }
        } else {
            if left.starts_with(&right) {
                return (Horizontal, i);
            }
        }
    }

    print_grid(grid);
    panic!("No symmetry found");
}

pub fn part_one(input: &str) -> Option<u32> {
    let grids = parse_input(input);

    let total: u32 = grids
        .iter()
        .map(find_reflection)
        .map(|(dir, i)| match dir {
            Symmetry::Horizontal => i as u32,
            Symmetry::Vertical => i as u32 * 100,
        })
        .sum();
    Some(total)
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
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
