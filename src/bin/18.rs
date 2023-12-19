aoc23_rust::solution!(18);
use std::collections::{HashMap, HashSet};

enum Heading {
    Up,
    Right,
    Down,
    Left,
}
struct Direction {
    heading: Heading,
    steps: u32,
    color: String,
}

fn count_enclosed_squares(floor: &HashMap<(i32, i32), String>, start: (i32, i32)) -> u32 {
    let mut count = 0;
    let mut inside_visited: HashSet<(i32, i32)> = HashSet::new();
    let mut border_visited: HashSet<(i32, i32)> = HashSet::new();
    let mut queue: Vec<(i32, i32)> = Vec::new();
    queue.push(start);

    while !queue.is_empty() {
        let (x, y) = queue.pop().unwrap();

        if floor.contains_key(&(x,y)) {
            if border_visited.contains(&(x, y)) {
                continue;
            } else {
                border_visited.insert((x, y));
            }
        } else {
            if inside_visited.contains(&(x, y)) {
                continue;
            } else {
                inside_visited.insert((x, y));
            }
        }

        count += 1;

        // we're inside and not on a border, so everything is valid
        if !floor.contains_key(&(x, y)) {
            queue.push((x + 1, y));
            queue.push((x - 1, y));
            queue.push((x, y + 1));
            queue.push((x, y - 1));
        } else {
            // we're on a border

            // check which sides are borders
            let border_right = floor.contains_key(&(x + 1, y));
            let border_up = floor.contains_key(&(x, y + 1));
            let border_left = floor.contains_key(&(x - 1, y));
            let border_down = floor.contains_key(&(x, y - 1));

            // check which sides are inside
            let inside_right = inside_visited.contains(&(x + 1, y));
            let inside_up = inside_visited.contains(&(x, y + 1));
            let inside_left = inside_visited.contains(&(x - 1, y));
            let inside_down = inside_visited.contains(&(x, y - 1));

            //1i#o 2i#. 3i#o 4iii 5iii 6iii
            // i#o  i##  ##o  ##i  i##  ###
            // i#o  iii  ooo  o#i  i#o  ooo

            match (
                border_left,
                border_up,
                border_right,
                border_down,
                inside_left,
                inside_up,
                inside_right,
                inside_down,
            ) {
                //1
                (false, true, false, true, true, false, false, false) => {
                    queue.push((x, y + 1));
                    queue.push((x, y - 1));
                }
                //2
                (false, true, true, false, true, false, false, _)
                | (false, true, true, false, _, false, false, true) => {
                    queue.push((x, y + 1));
                    queue.push((x + 1, y));
                    queue.push((x, y - 1));
                    queue.push((x - 1, y));
                }
                //3
                (true, true, false, false, false, false, false, false) => {
                    queue.push((x - 1, y));
                    queue.push((x, y + 1));
                }
                //4
                (true, false, false, true, false, true, _, false)
                | (true, false, false, true, false, _, true, false) => {
                    queue.push((x, y + 1));
                    queue.push((x + 1, y));
                    queue.push((x, y - 1));
                    queue.push((x - 1, y));
                }
                //5
                (false, false, true, true, true, _, false, false)
                | (false, false, true, true, _, true, false, false) => {
                    queue.push((x, y + 1));
                    queue.push((x + 1, y));
                    queue.push((x, y - 1));
                    queue.push((x - 1, y));
                }
                //6
                (true, false, true, false, false, true, false, false) => {
                    queue.push((x + 1, y));
                    queue.push((x, y + 1));
                    queue.push((x - 1, y));
                }

                //7o#i 8o#i 9o#i aooo booo cooo
                // o#i  o##  ##i  ##o  o##  ###
                // o#i  ooo  iii  i#o  o#i  iii

                //7
                (false, true, false, true, false, false, true, false) => {
                    queue.push((x, y + 1));
                    queue.push((x + 1, y));
                    queue.push((x, y - 1));
                }
                //8
                (false, true, true, false, false, false, false, false) => {
                    queue.push((x + 1, y));
                    queue.push((x, y + 1));
                }
                //9
                (true, true, false, false, false, false, true, _)
                | (true, true, false, false, false, false, _, true) => {
                    queue.push((x + 1, y));
                    queue.push((x, y + 1));
                }
                //a
                (true, false, false, true, false, false, false, false) => {
                    queue.push((x - 1, y));
                    queue.push((x, y - 1));
                }
                //b
                (false, false, true, true, false, false, false, false) => {
                    queue.push((x + 1, y));
                    queue.push((x, y - 1));
                }
                //c
                (true, false, true, false, false, false, false, true) => {
                    queue.push((x + 1, y));
                    queue.push((x - 1, y));
                }
                // we didn't find anything, but maybe we're sliding along a border?
                _ => {
                    /*println!( "({},{}):\nborders: {} {} {} {}\ninside: {} {} {} {}",
                        x,y,
                        border_left,
                        border_up,
                        border_right,
                        border_down,
                        inside_left,
                        inside_up,
                        inside_right,
                        inside_down
                    );*/
                    ()
                }
            }
        }
    }
    count
}

pub fn part_one(input: &str) -> Option<u32> {
    // parse each line of input like R ^ (#123456) into a Direction
    let directions = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let turn = match parts.next().unwrap() {
                "R" => Heading::Right,
                "L" => Heading::Left,
                "D" => Heading::Down,
                "U" => Heading::Up,
                _ => panic!("Invalid turn"),
            };
            let steps = parts.next().unwrap().parse::<u32>().unwrap();
            let color = parts.next().unwrap().to_string();
            Direction {
                heading: turn,
                steps,
                color,
            }
        })
        .collect::<Vec<Direction>>();

    // for each direction, move forward the number of steps
    let start: (i32, i32) = (0, 0);
    let mut floor: HashMap<(i32, i32), String> = HashMap::new();
    let destination = directions.iter().fold(start, |location, direction| {
        let mut x = location.0;
        let mut y = location.1;
        for _step in 0..direction.steps {
            (x, y) = match direction.heading {
                Heading::Up => (x, y + 1),
                Heading::Right => (x + 1, y),
                Heading::Down => (x, y - 1),
                Heading::Left => (x - 1, y),
            };
            // insert after move, we should close the loop at the end
            floor.insert((x, y), direction.color.clone());
        }
        (x, y)
    });
    assert!(destination == start);

    // count the number of tiles that are painted
    let count = count_enclosed_squares(&floor, (1, -1));

    Some(count)
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
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
