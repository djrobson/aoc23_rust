aoc23_rust::solution!(17);
use std::collections::HashMap;
use std::sync::OnceLock;

static GRID: OnceLock<Vec<Vec<u8>>> = OnceLock::new();

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Heading {
    Up,
    Down,
    Left,
    Right,
}

use Heading::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Bearing {
    x: i32,
    y: i32,
    heading: Heading,
    straight: i8,
}

impl Bearing {
    fn next_location_with_heading(&self, heading: Heading) -> (i32, i32) {
        let mut x = self.x;
        let mut y = self.y;
        match heading {
            Up => y -= 1,
            Down => y += 1,
            Left => x -= 1,
            Right => x += 1,
        }
        (x, y)
    }
    fn is_heading_on_board(&self, heading: Heading) -> bool {
        let (x, y) = self.next_location_with_heading(heading);
        x >= 0
            && x < GRID.get().unwrap()[0].len() as i32
            && y >= 0
            && y < GRID.get().unwrap().len() as i32
    }

    fn next_bearings(&self) -> Vec<Bearing> {
        let mut bearings = Vec::new();

        if self.straight < 2 && self.is_heading_on_board(self.heading) {
            let next_location = self.next_location_with_heading(self.heading);
            bearings.push(Bearing {
                x: next_location.0,
                y: next_location.1,
                heading: self.heading,
                straight: self.straight + 1,
            });
        }
        if self.is_heading_on_board(self.turn_left()) {
            let next_location = self.next_location_with_heading(self.turn_left());
            bearings.push(Bearing {
                x: next_location.0,
                y: next_location.1,
                heading: self.turn_left(),
                straight: 0,
            });
        }
        if self.is_heading_on_board(self.turn_right()) {
            let next_location = self.next_location_with_heading(self.turn_right());
            bearings.push(Bearing {
                x: next_location.0,
                y: next_location.1,
                heading: self.turn_right(),
                straight: 0,
            });
        }
        bearings
    }

    fn next_bearings2(&self) -> Vec<Bearing> {
        let mut bearings = Vec::new();

        if self.straight < 10 && self.is_heading_on_board(self.heading) {
            let next_location = self.next_location_with_heading(self.heading);
            bearings.push(Bearing {
                x: next_location.0,
                y: next_location.1,
                heading: self.heading,
                straight: self.straight + 1,
            });
        }
        if self.straight >= 3 && self.is_heading_on_board(self.turn_left()) {
            let next_location = self.next_location_with_heading(self.turn_left());
            bearings.push(Bearing {
                x: next_location.0,
                y: next_location.1,
                heading: self.turn_left(),
                straight: 0,
            });
        }
        if self.straight >= 3 && self.is_heading_on_board(self.turn_right()) {
            let next_location = self.next_location_with_heading(self.turn_right());
            bearings.push(Bearing {
                x: next_location.0,
                y: next_location.1,
                heading: self.turn_right(),
                straight: 0,
            });
        }
        bearings
    }

    fn turn_left(&self) -> Heading {
        match self.heading {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }

    fn turn_right(&self) -> Heading {
        match self.heading {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
    fn is_final_location(&self) -> bool {
        self.x == GRID.get().unwrap()[0].len() as i32 - 1
            && self.y == GRID.get().unwrap().len() as i32 - 1
    }
}

fn process_grid(bearing: Bearing, total_loss: u32, best_seen: &mut HashMap<Bearing, u32>) -> u32 {
    // check cost to get here
    let new_total_loss =
        total_loss + GRID.get().unwrap()[bearing.y as usize][bearing.x as usize] as u32;

    // if we already got here a better way, bail
    let old_best = *best_seen.get(&bearing).unwrap_or(&u32::MAX);
    if new_total_loss < old_best {
        best_seen.insert(bearing, new_total_loss);
    } else {
        return u32::MAX;
    }

    // if we're at the end, return the cost to get here
    if bearing.is_final_location() {
        println!("at end with score {}", new_total_loss);
        return new_total_loss;
    }

    // hacky shit to avoid infinite random walks
    if new_total_loss > GRID.get().unwrap().len() as u32 * 11 {
        return u32::MAX;
    }

    // find the minimum total cost of all options from bearings
    let min_bearings = bearing
        .next_bearings()
        .into_iter()
        .map(|b| process_grid(b, new_total_loss, best_seen))
        .min()
        .unwrap_or(u32::MAX);

    if min_bearings == u32::MAX {
        return u32::MAX;
    }

    if (bearing.x == 0 && bearing.y == 0)
        || (bearing.x == GRID.get().unwrap().len() as i32 && bearing.y == bearing.x)
    {
        println!(
            "at start of finish with score {}",
            new_total_loss + min_bearings
        );
    }
    new_total_loss + min_bearings
}

fn process_grid2(bearing: Bearing, total_loss: u32, best_seen: &mut HashMap<Bearing, u32>) -> u32 {
    // if we already got here a better way, bail
    let old_best = *best_seen.get(&bearing).unwrap_or(&u32::MAX);
    if total_loss < old_best {
        best_seen.insert(bearing, total_loss);
    } else {
        return u32::MAX;
    }

    // check cost to get here
    let new_total_loss =
        total_loss + GRID.get().unwrap()[bearing.y as usize][bearing.x as usize] as u32;

    if bearing.is_final_location() {
        if bearing.straight <= 3 {
            println!(
                "bad at end with score {} straight {}",
                new_total_loss, bearing.straight
            );
            return u32::MAX;
        } else {
            println!(
                "at end with score {} heading {:?} straight {}",
                new_total_loss, bearing.heading, bearing.straight
            );
            return new_total_loss;
        }
    }

    // hacky shit to avoid infinite random walks
    if new_total_loss > GRID.get().unwrap().len() as u32 * 11 {
        return u32::MAX;
    }

    // find the minimum total cost of all options from bearings
    let min_bearings = bearing
        .next_bearings2()
        .into_iter()
        .map(|b| process_grid2(b, new_total_loss, best_seen))
        .min()
        .unwrap_or(u32::MAX);

    if min_bearings == u32::MAX {
        return u32::MAX;
    }
    new_total_loss + min_bearings
}

pub fn part_one(input: &str) -> Option<u32> {
    let _ = GRID.set(parse_input(input));
    //assert!(GRID.get().unwrap().len() == GRID.get().unwrap()[0].len());
    let board_size = GRID.get().unwrap().len() as i32;
    let bearing: Bearing = Bearing {
        x: 0,
        y: 0,
        heading: Right,
        straight: 0,
    };
    let mut best_seen: HashMap<Bearing, u32> = HashMap::new();
    let _total_loss = process_grid(bearing, 0, &mut best_seen);

    // find the best finish
    let mut best_finish = u32::MAX;
    for bearing in [Up, Down, Left, Right] {
        for straight in 0..3 {
            let bearing = Bearing {
                x: board_size - 1,
                y: board_size - 1,
                heading: bearing,
                straight,
            };
            if best_seen.get(&bearing).is_some() {
                best_finish = best_finish.min(*best_seen.get(&bearing).unwrap());
            }
        }
    }

    // subtract start cost
    best_finish -= GRID.get().unwrap()[0][0] as u32;
    Some(best_finish)
}

pub fn part_two(input: &str) -> Option<u32> {
    let _ = GRID.set(parse_input(input));
    let bearing: Bearing = Bearing {
        x: 0,
        y: 0,
        heading: Right,
        straight: 0,
    };
    let mut best_seen: HashMap<Bearing, u32> = HashMap::new();
    let _total_loss = process_grid2(bearing, 0, &mut best_seen);

    // find the best finish
    let mut best_finish = u32::MAX;
    for bearing in [Down, Right] {
        for straight in 4..10 {
            let bearing = Bearing {
                x: GRID.get().unwrap()[0].len() as i32 - 1,
                y: GRID.get().unwrap().len() as i32 - 1,
                heading: bearing,
                straight,
            };
            if best_seen.get(&bearing).is_some() {
                best_finish = best_finish.min(*best_seen.get(&bearing).unwrap());
            }
        }
    }

    // subtract start cost
    best_finish -= GRID.get().unwrap()[0][0] as u32;
    best_finish +=
        GRID.get().unwrap()[GRID.get().unwrap().len() - 1][GRID.get().unwrap()[0].len() - 1] as u32;
    Some(best_finish)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
    #[test]
    fn test_part_two_2() {
        let result = part_two(
            "111111111111
999999999991
999999999991
999999999991
999999999991",
        );
        assert_eq!(result, Some(71));
    }
}
