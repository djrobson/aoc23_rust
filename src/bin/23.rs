aoc23_rust::solution!(23);
use glam::IVec2;

use std::collections::{HashSet, HashMap, VecDeque};

fn parse_input(input: &str) -> (Vec<Vec<char>>, IVec2) {
    let lines: Vec<&str> = input.lines().collect();
    let mut array_2d = Vec::new();

    let mut position_s = IVec2::splat(0);

    for (y, line) in lines.iter().enumerate() {
        let elements: Vec<char> = line.chars().collect();
        if let Some(x) = elements.iter().position(|&c| c == 'S') {
            position_s = IVec2::new(x as i32, y as i32);
        }
        array_2d.push(elements);
    }

    (array_2d, position_s)
}

fn get_vertices(grid: &Vec<Vec<char>>) -> Vec<IVec2> {
    let mut vertices = Vec::new();

    for (y, line) in grid.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c == '#' {
                continue;
            }

            // get the characters up, left, down, right from xy, if at least 2 of them are in "<>^v" then xy is a vertex
            let mut count = 0;
            if x > 0 && "<>^v".contains(grid[y][x-1]) {
                count += 1;
            }
            if x < grid[0].len() - 1 && "<>^v".contains(grid[y][x+1]) {
                count += 1;
            }
            if y > 0 && "<>^v".contains(grid[y-1][x]) {
                count += 1;
            }
            if y < grid.len() - 1 && "<>^v".contains(grid[y+1][x]) {
                count += 1;
            }

            if c == 'S' || (y == grid.len() -1 && x == grid[0].len() - 2) {
                count += 2; // start and end are a vertex
            }

            if count >= 2
            {
                vertices.push(IVec2::new(x as i32, y as i32));
            }
        }
    }

    vertices
}

fn get_next_positions(grid: &Vec<Vec<char>>, position: IVec2) -> Vec<IVec2> {
    let mut next_positions = Vec::new();

    let x: usize = position.x as usize;
    let y: usize = position.y as usize;

    // if current position in < > ^ v then only go left right up or down, respectively
    match grid[y][x] {
        '<' => {
            if x > 0 && grid[y][x-1] != '#' {
                next_positions.push(position + IVec2::NEG_X);
            }
        },
        '>' => {
            if x < grid[0].len() - 1 && grid[y][x+1] != '#' {
                next_positions.push(position + IVec2::X);
            }
        },
        '^' => {
            if y > 0 && grid[y-1][x] != '#' {
                next_positions.push(position + IVec2::NEG_Y);
            }
        },
        'v' => {
            if y < grid.len() - 1 && grid[y+1][x] != '#' {
                next_positions.push(position + IVec2::Y);
            }
        },
        _ => {
            if x > 0 && grid[y][x-1] != '#' {
                next_positions.push(position + IVec2::NEG_X);
            }
            if x < grid[0].len() - 1 && grid[y][x+1] != '#' {
                next_positions.push(position + IVec2::X);
            }
            if y > 0 && grid[y-1][x] != '#' {
                next_positions.push(position + IVec2::NEG_Y);
            }
            if y < grid.len() - 1 && grid[y+1][x] != '#' {
                next_positions.push(position + IVec2::Y);
            }
        }
    }

    // else go to any direction that's not a #

    next_positions
}

// print dominator graph in DOT format
fn print_dominator_graph(dominator_graph: &HashMap<IVec2,Vec<(IVec2, usize)>>) {
    println!("digraph G {{");
    for (vertex, dominated) in dominator_graph {
        for (dominated_vertex, _) in dominated {
            println!("  \"{}\" -> \"{}\"", vertex, dominated_vertex);
        }
    }
    println!("}}");
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, start) = parse_input(input);
    let end = (grid[0].len() - 1, grid.len() - 2); // example and input ending is just left of the bottom right corner

    let vertices = get_vertices(&grid);

    // find the topological dominator graph for all vertices on grid
    // for each pair of vertices, find any path between them that doesn't go through any other vertex
    // if there is a path from the first vertex to the second, then the first vertex is a dominator of the second
    // append the second vertex to the dominated list of the first vertex
    let mut dominator_graph: HashMap<IVec2,Vec<(IVec2, usize)>> = HashMap::new();
    for vertex1 in &vertices {
        for vertex2 in &vertices {
            if (vertex1.x, vertex1.y) == ( vertex2.x, vertex2.y) {
                continue;
            }

            let mut path = HashSet::new();
            let mut path_options = VecDeque::new();
            path_options.push_back(*vertex1);

            while let Some(position) = path_options.pop_front() {
                if path.contains(&position) {
                    continue;
                }

                path.insert(position);

                if position == *vertex2 {
                    // check is there's already a path between the two vertices and keep the longer one???
                    dominator_graph.entry(*vertex1).or_insert(Vec::new()).push((*vertex2, path.len()));
                    break;
                }

                for next_position in get_next_positions(&grid, position) {
                    path_options.push_back(next_position);
                }
            }
        }
    }

    print_dominator_graph(&dominator_graph);

    // for each vertes in the dominator graph do a depth first search to find the longest path
    // if you reach a vertex for the first time then record the path length and vertices traversed
    // if you reach a vertex for the second time then check if the path length is longer than the previous one
    // if the vertex is the end vertex, then the cost is the length of the path from the start vertex
    // if the vertex is not the end vertex, then the cost is the length of the path from the start vertex plus the highest cost of the dominated vertices

    None
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
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
