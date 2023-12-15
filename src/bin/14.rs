use std::collections::HashMap;
use std::fmt::{Display, Formatter};

aoc23_rust::solution!(14);

#[derive(PartialEq, Eq, Clone, Hash)]
enum Rock {
    Round,
    Cube,
    Empty,
}

impl Display for Rock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Rock::Round => write!(f, "O"),
            Rock::Cube => write!(f, "#"),
            Rock::Empty => write!(f, "."),
        }
    }
}

enum Tilt {
    North,
    South,
    East,
    West,
}

fn get_load(rock_grid: &Vec<Vec<Rock>>) -> u32 {
    let length = rock_grid.len();
    rock_grid
        .iter()
        .enumerate()
        .map(|(row_id, val)| {
            let row_weight: u32 = (length as u32) - row_id as u32;
            let round_in_row = val.iter().filter(|&r| *r == Rock::Round).count() as u32;
            row_weight * round_in_row
        })
        .sum::<u32>()
}

fn print_grid(rock_grid: &Vec<Vec<Rock>>) {
    for row in rock_grid {
        for rock in row {
            print!("{}", rock);
        }
        println!();
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    // parse input like @14.txt into a 2d array of Rock
    // O becomes Round, # becomes Cube, . becomes Empty
    let mut rock_grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'O' => Rock::Round,
                    '#' => Rock::Cube,
                    '.' => Rock::Empty,
                    _ => panic!("unexpected character in input: {}", c),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    tilt_board(&mut rock_grid, Tilt::North);
    print_grid(&rock_grid);
    Some(get_load(&rock_grid))
}

fn tilt_board(rock_grid: &mut Vec<Vec<Rock>>, tilt: Tilt) {
    let length = rock_grid.len();
    match tilt {
        Tilt::North => {
            for row in 0..length {
                for col in 0..length {
                    // for each Round rock, slide it to a lower index row until it bumps into the top row or a Square rock
                    if rock_grid[row][col] == Rock::Round {
                        let mut new_row = row;
                        while new_row > 0 {
                            if rock_grid[new_row - 1][col] != Rock::Empty {
                                break;
                            }
                            rock_grid[new_row - 1][col] = Rock::Round;
                            rock_grid[new_row][col] = Rock::Empty;
                            new_row -= 1;
                        }
                    }
                }
            }
        }
        Tilt::West => {
            #[allow(clippy::needless_range_loop)]
            for row in 0..length {
                for col in 0..length {
                    // for each Round rock, slide it to a lower index row until it bumps into the top row or a Square rock
                    if rock_grid[row][col] == Rock::Round {
                        let mut new_col = col;
                        while new_col > 0 {
                            if rock_grid[row][new_col - 1] != Rock::Empty {
                                break;
                            }
                            rock_grid[row][new_col - 1] = Rock::Round;
                            rock_grid[row][new_col] = Rock::Empty;
                            new_col -= 1;
                        }
                    }
                }
            }
        }
        Tilt::South => {
            for row in (0..length).rev() {
                for col in 0..length {
                    // for each Round rock, slide it to a lower index row until it bumps into the top row or a Square rock
                    if rock_grid[row][col] == Rock::Round {
                        let mut new_row = row;
                        while new_row < length - 1 {
                            if rock_grid[new_row + 1][col] != Rock::Empty {
                                break;
                            }
                            rock_grid[new_row + 1][col] = Rock::Round;
                            rock_grid[new_row][col] = Rock::Empty;
                            new_row += 1;
                        }
                    }
                }
            }
        }
        Tilt::East => {
            #[allow(clippy::needless_range_loop)]
            for row in 0..length {
                for col in (0..length).rev() {
                    // for each Round rock, slide it to a lower index row until it bumps into the top row or a Square rock
                    if rock_grid[row][col] == Rock::Round {
                        let mut new_col = col;
                        while new_col < length - 1 {
                            if rock_grid[row][new_col + 1] != Rock::Empty {
                                break;
                            }
                            rock_grid[row][new_col + 1] = Rock::Round;
                            rock_grid[row][new_col] = Rock::Empty;
                            new_col += 1;
                        }
                    }
                }
            }
        }
    }
}

fn tilt_around(rock_grid: &mut Vec<Vec<Rock>>) {
    tilt_board(rock_grid, Tilt::North);
    tilt_board(rock_grid, Tilt::West);
    tilt_board(rock_grid, Tilt::South);
    tilt_board(rock_grid, Tilt::East);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rock_grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'O' => Rock::Round,
                    '#' => Rock::Cube,
                    '.' => Rock::Empty,
                    _ => panic!("unexpected character in input: {}", c),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut seen_grids: HashMap<Vec<Vec<Rock>>, usize> = HashMap::new();
    //seen_grids.insert(rock_grid.clone(), 0);

    'loops: for round in 0..1_000_000_000 {
        tilt_around(&mut rock_grid);
        if seen_grids.contains_key(&rock_grid) {
            let loop_start = seen_grids.get(&rock_grid).unwrap();
            println!("found a loop at round {} back to {}", round, loop_start);
            // fast forward to the end of the iterations
            let loop_length = round - loop_start;
            let remaining_rounds = 1_000_000_000 - round;
            let remaining_loops = remaining_rounds / loop_length;
            let remaining_rounds_after_loops = remaining_rounds % loop_length;
            println!(
                "remaining rounds: {} loops: {}",
                remaining_rounds, remaining_loops
            );
            for last_rounds in 0..remaining_rounds_after_loops - 1 {
                tilt_around(&mut rock_grid);
                println!("last_round {} score {}", last_rounds, get_load(&rock_grid));
            }
            break 'loops;
        } else {
            seen_grids.insert(rock_grid.clone(), round);
        }
    }
    Some(get_load(&rock_grid))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
