use std::fmt::{Display, Formatter};

aoc23_rust::solution!(14);

#[derive(PartialEq)]
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
    rock_grid.iter().enumerate().map(|(row_id, val)| {
        let row_weight: u32 = (length as u32) - row_id as u32;
        let round_in_row = val.iter().filter(|&r| *r == Rock::Round).count() as u32;
        row_weight * round_in_row
    }).sum::<u32>()
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
    let mut rock_grid = input.lines().map(|line| {
        line.chars().map(|c| match c {
            'O' => Rock::Round,
            '#' => Rock::Cube,
            '.' => Rock::Empty,
            _ => panic!("unexpected character in input: {}", c),
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    tilt_board(&mut rock_grid, Tilt::North);
    print_grid(&rock_grid);
    Some(get_load(&rock_grid)) 
}

fn tilt_board( rock_grid: &mut Vec<Vec<Rock>>, tilt: Tilt) {
    let length = rock_grid.len();
    match tilt {
        Tilt::North => {
            for row in 0..length {
                for col in 0..length {
                    // for each Round rock, slide it to a lower index row until it bumps into the top row or a Square rock
                    if rock_grid[row][col] == Rock::Round {
                        let mut new_row = row;
                        while new_row > 0 {
                            if rock_grid[new_row-1][col] != Rock::Empty {
                                break;
                            }
                            rock_grid[new_row-1][col] = Rock::Round;
                            rock_grid[new_row][col] = Rock::Empty;
                            new_row -= 1;
                        }
                    }
                }
            }
        },
        _ => panic!("not implemented"),
    }
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
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
