aoc23_rust::solution!(21);
use std::collections::{HashSet, VecDeque};

fn parse_input(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let mut grid = Vec::new();
    let mut s_location = None;

    for (i, line) in input.lines().enumerate() {
        let line = line;
        let row: Vec<char> = line.chars().collect();
        if let Some(j) = row.iter().position(|&c| c == 'S') {
            s_location = Some((i, j));
        }
        grid.push(row);
    }

    (grid, s_location.unwrap())
}

fn get_neighbors(grid: &Vec<Vec<char>>, row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let rows = grid.len();
    let cols = grid[0].len();

    if row > 0 && grid[row - 1][col] != '#' {
        neighbors.push((row - 1, col));
    }
    if row < rows - 1 && grid[row + 1][col] != '#' {
        neighbors.push((row + 1, col));
    }
    if col > 0 && grid[row][col - 1] != '#' {
        neighbors.push((row, col - 1));
    }
    if col < cols - 1 && grid[row][col + 1] != '#' {
        neighbors.push((row, col + 1));
    }

    neighbors
}

fn get_grid_on_next_step(
    grid: &Vec<Vec<char>>,
    locations: &HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let mut new_locations = HashSet::new();

    for (row, col) in locations {
        let neighbors = get_neighbors(&grid, *row, *col);
        for (n_row, n_col) in neighbors {
            new_locations.insert((n_row, n_col));
        }
    }

    new_locations
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, s_location) = parse_input(input);
    let (s_row, s_col) = s_location;

    let mut locations = HashSet::new();
    locations.insert((s_row, s_col));

    for _step in 0..64 {
        locations = get_grid_on_next_step(&grid, &locations);
    }

    Some(locations.len() as u32)
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
