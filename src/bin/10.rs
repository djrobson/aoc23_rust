aoc23_rust::solution!(10);

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;

fn find_directions_from_start(
    input_array: &Vec<Vec<char>>,
    x: usize,
    y: usize,
) -> (Direction, Direction) {
    let mut dirs: Vec<Direction> = Vec::new();
    if y > 1
        && x < input_array[y].len()
        && (input_array[y - 1][x] == '|'
            || input_array[y - 1][x] == '7'
            || input_array[y - 1][x] == 'F')
    {
        dirs.push(Up);
    }

    if y < input_array.len()
        && x < input_array[y].len()
        && (input_array[y + 1][x] == '|'
            || input_array[y + 1][x] == 'J'
            || input_array[y + 1][x] == 'L')
    {
        dirs.push(Down);
    }

    if y < input_array.len()
        && x > 1
        && (input_array[y][x - 1] == '-'
            || input_array[y][x - 1] == 'F'
            || input_array[y][x - 1] == 'L')
    {
        dirs.push(Left);
    }

    if y < input_array.len()
        && x < input_array[y].len()
        && (input_array[y][x + 1] == '-'
            || input_array[y][x + 1] == 'J'
            || input_array[y][x + 1] == '7')
    {
        dirs.push(Right);
    }

    assert!(dirs.len() == 2);
    (dirs[0], dirs[1])
}

fn vector_from_direction(dir: &Direction) -> (i8, i8) {
    match *dir {
        Up => (0, -1),
        Down => (0, 1),
        Left => (-1, 0),
        Right => (1, 0),
    }
}

fn get_shape_from_direction(dirs: (Direction, Direction)) -> char {
    match dirs {
        (Up, Down) => '|',
        (Up, Left) => 'F',
        (Up, Right) => '7',
        (Down, Up) => '│',
        (Down, Left) => '7',
        (Down, Right) => 'F',
        (Left, Up) => 'J',
        (Left, Down) => '7',
        (Left, Right) => '─',
        (Right, Up) => 'L',
        (Right, Down) => 'F',
        (Right, Left) => '─',
        _ => panic!("bad direction tuple"),
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

fn get_input_array(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn get_start_location(input_array: &[Vec<char>]) -> (usize, usize) {
    input_array
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == 'S').map(|x| (x, y)))
        .expect("didn't find start location")
}

fn get_pipe_list(input: &str) -> Vec<(usize, usize)> {
    let input_array = get_input_array(input);
    let sloc = get_start_location(&input_array);
    let mut visited: Vec<(usize, usize)> = Vec::new();
    let mut cur_loc = sloc;
    let mut next_vector: (i8, i8);
    let mut next_direction = find_directions_from_start(&input_array, sloc.0, sloc.1).0;

    loop {
        visited.push(cur_loc);
        next_vector = vector_from_direction(&next_direction);
        cur_loc = get_next_loc(&cur_loc, next_vector);
        if cur_loc == sloc {
            break;
        }
        next_direction = get_next_direction(&next_direction, input_array[cur_loc.1][cur_loc.0]);
    }
    visited
}

pub fn part_one(input: &str) -> Option<u32> {
    let visited = get_pipe_list(input);

    let length = visited.len() as u32;
    Some(length.div_ceil(2))
}

pub fn part_two(input: &str) -> Option<u32> {
    let visited = get_pipe_list(input);

    let mut input_array: Vec<Vec<char>> = get_input_array(input);

    /*
    let min_x = visited.iter().min_by_key(|&(x, _)| x).unwrap().0;
    let max_x = visited.iter().max_by_key(|&(x, _)| x).unwrap().0;
    let min_y = visited.iter().min_by_key(|&(_, y)| y).unwrap().1;
    let max_y = visited.iter().max_by_key(|&(_, y)| y).unwrap().1;
    */

    let min_x = 0;
    let max_x = input_array[0].len();
    let min_y = 0;
    let max_y = input_array.len();

    let mut enclosed_points = 0;
    let sloc = get_start_location(&input_array);
    let sdir = find_directions_from_start(&input_array, sloc.0, sloc.1);
    let sshape = get_shape_from_direction(sdir);
    input_array[sloc.1][sloc.0] = sshape;

    #[allow(clippy::needless_range_loop)] 
    for y in min_y..max_y {
        let mut is_inside = false;
        for x in min_x..max_x {
            let tile = input_array[y][x];
            //if is_inside && (tile == '.' || !visited.contains(&(x, y))) {
            if is_inside && !visited.contains(&(x, y)) {
                enclosed_points += 1;
            }
            if visited.contains(&(x, y)) {
                is_inside = match (is_inside, tile) {
                    (true, '|') => false,
                    (false, '|') => true,
                    (true, 'F') => false,
                    (false, 'F') => true,
                    (true, '7') => false,
                    (false, '7') => true,
                    (true, _) => true,
                    (false, _) => false,
                };
            }
        }
    }

    Some(enclosed_points)
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
        let input: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let result = part_two(input);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_1() {
        let input: &str = "..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........";
        let result = part_two(input);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_2() {
        let input: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let result = part_two(input);
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_3() {
        let input: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        let result = part_two(input);
        assert_eq!(result, Some(10));
    }
}
