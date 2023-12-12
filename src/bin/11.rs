aoc23_rust::solution!(11);

use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> Option<u32> {
    let mut coordinates = HashSet::new();
    let input_lines: Vec<&str> = input.lines().collect();
    for (y, line) in input_lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                coordinates.insert((x, y));
            }
        }
    }

    // consider pre-loading row and col list, then removing collisions during nitial parsing
    let mut row_stretch: Vec<usize> = Vec::new();
    let mut stretch_down: usize = 0;
    for row in 0..input_lines.len() {

        if input_lines[row].chars().filter(|c| c == &'#').count() == 0 {
            stretch_down += 1;
        }
        row_stretch.push(stretch_down);
    }
    let mut col_stretch: Vec<usize> = Vec::new();
    let mut stretch_right: usize = 0;
    for col in 0..input_lines[0].len() {
        let mut found = false;
        for row in 0..input_lines.len() {
            if input_lines[row].chars().nth(col).unwrap()  == '#' {
                found = true;
                break;
            }
        }
        if !found {
            stretch_right += 1;
        }
        col_stretch.push(stretch_right);
    }

    let mut coodinates_stretched: HashSet<(usize,usize)> = HashSet::new();
    for galaxy in coordinates.iter() {
        coodinates_stretched.insert( (galaxy.0 + col_stretch[galaxy.0], galaxy.1 + row_stretch[galaxy.1]));
    }

    let mut total_min_distance = 0;
    for gal1 in coodinates_stretched.iter() {
        for gal2 in coodinates_stretched.iter() {
            let distance = (gal1.0 as i32 - gal2.0 as i32).abs() + (gal1.1 as i32 - gal2.1 as i32).abs();
            //println!("distance from {:?} to {:?} is {}", gal1, gal2, distance);
            total_min_distance += distance;
        }
    }

    // we counted everything twice
    total_min_distance /= 2;

    Some(total_min_distance as u32)
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
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
