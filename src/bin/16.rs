aoc23_rust::solution!(16);

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Space {
    Empty,
    Vert,
    Horiz,
    UpRight,
    UpLeft,
}
use Space::*;

fn get_next_location(grid: &Vec<Vec<Space>>, current: (usize, usize), dir: Direction) -> Option<(usize,usize, Direction)> {
    match dir {
        Up => if current.1 > 0 {
                Some((current.0, current.1 - 1, Up)) }
            else {
                None
            },
        Down => if current.1 < grid.len() - 1 {
                Some((current.0, current.1 + 1,Down)) }
            else {
                None
            },
        Left => if current.0 > 0 {
                Some((current.0 - 1, current.1,Left)) }
            else {
                None
            },
        Right => if current.0 < grid[0].len() - 1 {
                Some((current.0 + 1, current.1,Right)) }
            else {
                None
            },
    }
}

fn get_next_direction(grid: &Vec<Vec<Space>>, current: (usize, usize), dir: Direction) -> Vec<Direction> {
    let mut next_directions = Vec::new();
    match (grid[current.1][current.0] ,dir){
        (Empty,_) => next_directions.push(dir),
        (Horiz,Up) | (Horiz,Down) => {
            next_directions.push(Left);
            next_directions.push(Right);
        }
        (Horiz,_) => next_directions.push(dir),
        (Vert,Left) | (Vert,Right) => {
            next_directions.push(Up);
            next_directions.push(Down);
        }
        (Vert,_) => next_directions.push(dir),
        (UpLeft,Up) => next_directions.push(Left),
        (UpLeft,Left) => next_directions.push(Up),
        (UpLeft,Down) => next_directions.push(Right),
        (UpLeft,Right) => next_directions.push(Down),
        (UpRight,Up) => next_directions.push(Right),
        (UpRight,Left) => next_directions.push(Down),
        (UpRight,Down) => next_directions.push(Left),
        (UpRight,Right) => next_directions.push(Up),
        //_ =>  panic!("Invalid character")   
    }
    next_directions
}


fn find_nodes_in_path(grid: &Vec<Vec<Space>>, current: (usize, usize), dir: Direction, visited: &mut HashSet<((usize,usize),Direction)>) -> () {
/*
    be in a location with a direction
    check if we've been here in visited
    add location and vec to visited
    get next directions
    for each direction check if valid
    go down the first direction with caching in visited
    go down the second direction with caching in visited
*/
    if visited.contains(&(current, dir)) {
        return;
    }
    visited.insert((current, dir));
    let next_directions = get_next_direction(grid, current, dir);
    next_directions.iter().for_each(|next_dir| {
        let next_location = get_next_location(grid, current, *next_dir);
        if next_location.is_some() {
            find_nodes_in_path(grid, (next_location.unwrap().0, next_location.unwrap().1), *next_dir, visited);
        }
    });
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<Space>> =     input.lines().map(|line| {
        line.chars().map(|c| {
            match c {
                '.' => Empty,
                '|' => Vert,
                '\\' => UpLeft,
                '/' => UpRight,
                '-' => Horiz,
                _ => panic!("Invalid character"),
            }
        }).collect()
    }).collect();

    let mut visited: HashSet<((usize,usize),Direction)> = HashSet::new();
    find_nodes_in_path(&grid, (0,0), Right, &mut visited);

    let energized = visited.iter().map(|((x,y),_)| (x,y)).collect::<HashSet<_>>();

    Some(energized.len() as u32)
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
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
