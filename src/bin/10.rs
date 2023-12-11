aoc23_rust::solution!(10);

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;

fn find_any_direction_from_start(input_array: &Vec<Vec<char>>, x: &usize, y: &usize) -> Direction {
    if y > &1 && input_array.get(*y-1).and_then(|row| row.get(*x)).is_some(){
        return Up;
    } else if y+1 <= input_array.len() && input_array.get(*y+1).and_then(|row| row.get(*x)).is_some(){
        return Down;
    } else if x > &1 && input_array.get(*y).and_then(|row| row.get(*x-1)).is_some(){
        return Left;
    } else if x+1 <= input_array[*y].len() && input_array.get(*y).and_then(|row| row.get(*x+1)).is_some(){
        return Right;
    } else {
        panic!("didn't find starting position");
    }
    
}

fn vector_from_direction(dir: &Direction) -> (i8, i8) {
    match *dir {
        Up => (0, -1),
        Down => (0, 1),
        Left => (-1, 0),
        Right => (1, 0),
    }
}

fn get_next_direction(dir: &Direction, shape: char) -> Direction {
    match (dir, shape) {
        (_, '.') => panic!("found a dead"),
        (Up, 'F') => Right,
        (Left, 'F') => Down,
        (Up, '7') => Left,
        (Right, '7') => Down,
        (Down, 'J') => Left,
        (Right, 'J') => Up,
        (Down, 'L') => Right,
        (Left, 'L') => Up,
        (Down, '|') => Down,
        (Up, '|') => Up,
        (Left, '-') => Left,
        (Right, '-') => Right,
        _ => panic!("unexpected {:?} {}", dir, shape),
    }
}

fn get_next_loc(cur_loc: &(usize, usize), vector: (i8, i8)) -> (usize, usize) {
    (
        cur_loc.0.checked_add_signed(vector.0 as isize).unwrap(),
        cur_loc.1.checked_add_signed(vector.1 as isize).unwrap(),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let input_array: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let sloc = input_array
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == 'S').map(|x| (x, y)))
        .expect("didn't find start location");

    let mut visited: Vec<(usize, usize)> = Vec::new();
    let mut cur_loc = sloc;
    let mut next_vector: (i8,i8);
    let mut next_direction = find_any_direction_from_start(&input_array, &sloc.1, &sloc.0);

    loop {
        visited.push(cur_loc);
        next_vector = vector_from_direction(&next_direction);
        cur_loc = get_next_loc(&cur_loc, next_vector);
        if cur_loc == sloc {
            break;
        }
        next_direction = get_next_direction(&next_direction, input_array[cur_loc.1][cur_loc.0]);
    }

    let length = visited.len() as u32;
    Some(length.div_ceil(2))
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
