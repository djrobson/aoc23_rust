aoc23_rust::solution!(24);

use std::convert;

use glam::Vec3;

fn parse_input(input: &str) -> (Vec<(Vec3, Vec3)>) {
    let mut loc_vels = Vec::new();

    for line in input.lines() {
        let halves: Vec<&str> = line.split('@').collect();
        let first: Vec<f32> = halves[0].split(',').map(|s| s.trim().parse().unwrap()).collect();
        let second: Vec<f32> = halves[1].split(',').map(|s| s.trim().parse().unwrap()).collect();
        loc_vels.push((
            Vec3::new(first[0], first[1], first[2]),
            Vec3::new(second[0], second[1], second[2]),
        ));
    }
    loc_vels
}
struct Line {
    slope: f32,
    intercept: f32,
}

fn find_intersection_2d(line1: &Line, line2: &Line) -> Option<Vec3> {
    if line1.slope == line2.slope {
        // The lines are parallel and do not intersect
        None
    } else {
        let x = (line2.intercept - line1.intercept) / (line1.slope - line2.slope);
        let y = line1.slope * x + line1.intercept;
        Some(Vec3::new(x, y, 0.0))
    }
}

fn convert_to_line(loc: &Vec3, vel: &Vec3) -> Line {
    Line {
        slope: vel.y / vel.x,
        intercept: loc.y - loc.x * vel.y / vel.x,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    // convert a location and velocity vector into a line
    let lines: Vec<Line> = parse_input(input)
        .iter()
        .map(|(loc, vel)| convert_to_line(loc, vel))
        .collect();

    let mut count = 0;

    for line1 in 0..lines.len() {
        for line2 in line1 + 1..lines.len() {
            if let Some(intersection) = find_intersection_2d(&lines[line1], &lines[line2]) {
                // if the intersection occurs within the square of x=7,y=7 and x=27,y=27
                if intersection.x >= 7.0
                    && intersection.x <= 27.0
                    && intersection.y >= 7.0
                    && intersection.y <= 27.0
                {
                    count += 1;
                }
            }
        }
    }
    Some(count)
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
