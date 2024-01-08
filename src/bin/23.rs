aoc23_rust::solution!(23);
use glam::IVec2;
use std::collections::{HashMap, VecDeque};

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.trim().bytes().collect())
        .collect()
}

fn get_vertices(grid: &Vec<Vec<u8>>) -> Vec<IVec2> {
    let mut vertices = Vec::new();

    for (y, line) in grid.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c == b'#' {
                continue;
            }

            // get the characters up, left, down, right from xy, if at least 2 of them are in "<>^v" then xy is a vertex
            let mut count = 0;
            if x > 0 && "<>^v".contains(grid[y][x - 1] as char) {
                count += 1;
            }
            if x < grid[0].len() - 1 && "<>^v".contains(grid[y][x + 1] as char) {
                count += 1;
            }
            if y > 0 && "<>^v".contains(grid[y - 1][x] as char) {
                count += 1;
            }
            if y < grid.len() - 1 && "<>^v".contains(grid[y + 1][x] as char) {
                count += 1;
            }

            if (y == 0 && x == 1) || (y == grid.len() - 1 && x == grid[0].len() - 2) {
                count += 2; // start and end are a vertex
            }

            if count >= 2 {
                vertices.push(IVec2::new(x as i32, y as i32));
            }
        }
    }

    vertices
}

fn check_grid_for_forks(grid: &Vec<Vec<u8>>) -> bool {
    let mut found_fork = false;
    for (y, line) in grid.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c == b'#' {
                continue;
            }

            // get the characters up, left, down, right from xy, if at least 2 of them are in "<>^v" then xy is a vertex
            let mut count = 0;
            if x > 0 && grid[y][x - 1] == b'.' {
                count += 1;
            }
            if x < grid[0].len() - 1 && grid[y][x + 1] == b'.' {
                count += 1;
            }
            if y > 0 && grid[y - 1][x] == b'.' {
                count += 1;
            }
            if y < grid.len() - 1 && grid[y + 1][x] == b'.' {
                count += 1;
            }

            if count >= 3 {
                println!("fork at {},{}", x, y);
                found_fork = true;
            }
        }
    }
    found_fork
}

fn get_edges_from_node(grid: &Vec<Vec<u8>>, node: IVec2) -> Vec<IVec2> {
    let mut egress = Vec::new();

    let x: usize = node.x as usize;
    let y: usize = node.y as usize;

    // for each direction check if there is a path leaving the current node
    // if there is a path, then follow it until you reach a vertex
    if (node + IVec2::X).x < grid[0].len() as i32 && grid[y][x + 1] == b'>' {
        egress.push(node + IVec2::X);
    }
    if (node - IVec2::X).x >= 0 && grid[y][x - 1] == b'<' {
        egress.push(node - IVec2::X);
    }
    if (node + IVec2::Y).y < grid.len() as i32 && grid[y + 1][x] == b'v' {
        egress.push(node + IVec2::Y);
    }
    if (node - IVec2::Y).y >= 0 && grid[y - 1][x] == b'^' {
        egress.push(node - IVec2::Y);
    }

    egress
}

fn follow_path_until_vertex(grid: &[Vec<u8>], start: &IVec2, vertices: &[IVec2]) -> (IVec2, usize) {
    let mut path_length = 1;
    let mut current_node = *start;
    let mut previous_node = match grid[start.y as usize][start.x as usize] {
        b'>' => current_node - IVec2::X,
        b'<' => current_node + IVec2::X,
        b'v' => current_node - IVec2::Y,
        b'^' => current_node + IVec2::Y,
        _ => panic!("invalid start node"),
    };

    assert!(grid[current_node.y as usize][current_node.x as usize] != b'#');

    // continue following the path of b'.' until you reach a vertex, don't back track
    loop {
        let available_edges: Vec<IVec2> = [IVec2::X, IVec2::Y, -IVec2::X, -IVec2::Y]
            .iter()
            .flat_map(|edge| {
                let possible_next = current_node + *edge;
                if grid[possible_next.y as usize][possible_next.x as usize] != b'#'
                    && possible_next != previous_node
                {
                    Some(possible_next)
                } else {
                    None
                }
            })
            .collect::<Vec<IVec2>>();

        if available_edges.len() != 1 {
            panic!("bad path at {},{}", current_node.x, current_node.y);
        }

        path_length += 1;
        if vertices.contains(&available_edges[0]) {
            current_node = available_edges[0];
            break;
        }

        // update cursors and keep walking
        previous_node = current_node;
        current_node = available_edges[0];
    }

    (current_node, path_length)
}

// print dominator graph in DOT format
#[allow(dead_code)]
fn print_graph(graph: &HashMap<IVec2, HashMap<IVec2, usize>>) {
    println!("digraph G {{");
    for vertex in graph.keys() {
        for other_vertex in graph.get(vertex).unwrap().keys() {
            println!(
                "  \"{}\" -> \"{}\" [weight={}]",
                vertex,
                other_vertex,
                graph.get(vertex).unwrap().get(other_vertex).unwrap()
            );
        }
    }
    println!("}}");
}

fn get_graph(vertices: &[IVec2], grid: &Vec<Vec<u8>>) -> HashMap<IVec2, HashMap<IVec2, usize>> {
    let mut graph: HashMap<IVec2, HashMap<IVec2, usize>> = HashMap::new();

    let mut nodes: VecDeque<IVec2> = VecDeque::new();
    let start = IVec2::new(1, 0);

    // walk the vertices starting from the start node
    nodes.push_back(start);
    while let Some(vertex) = nodes.pop_front() {
        if graph.contains_key(&vertex) {
            continue; // we already processed this one
        }

        // find all adjacent vertices with their cost
        let egress = get_edges_from_node(grid, vertex);
        let edges = egress
            .iter()
            .map(|&edge| follow_path_until_vertex(grid, &edge, vertices))
            .collect::<Vec<(IVec2, usize)>>();

        let mut adj_nodes = HashMap::new();
        for edge in edges {
            adj_nodes.insert(edge.0, edge.1);

            // add the adjacent vertices to the queue
            nodes.push_back(edge.0);
        }
        graph.insert(vertex, adj_nodes);
    }

    graph
}

fn find_worst_cost(
    graph: &HashMap<IVec2, HashMap<IVec2, usize>>,
    start: &IVec2,
    end: &IVec2,
) -> usize {
    let mut worst_found: HashMap<IVec2, usize> = HashMap::new();
    //worst_found.insert(*start, 0);
    let mut process: VecDeque<(IVec2, usize)> = VecDeque::new();
    process.push_back((*start, 0));
    while let Some((vertex, cost)) = process.pop_front() {
        if worst_found.contains_key(&vertex) {
            // update vertex if we found something worse
            if &cost > worst_found.get(&vertex).unwrap() {
                worst_found.insert(vertex, cost);
            } else {
                continue;
            }
        } else {
            worst_found.insert(vertex, cost);
        }
        for (adj_vertex, adj_cost) in graph.get(&vertex).unwrap() {
            process.push_back((*adj_vertex, cost + adj_cost));
        }
    }
    *worst_found.get(end).unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    assert!(!check_grid_for_forks(&grid));

    // example and input ending is just left of the bottom right corner
    let start = IVec2::new(1, 0);
    let end = IVec2::new((grid[0].len() - 2) as i32, (grid.len() - 1) as i32);

    let mut vertices = get_vertices(&grid);
    vertices.push(start);
    vertices.push(end);

    // find theweighted graph for all vertices on grid
    let graph = get_graph(&vertices, &grid);

    //print_graph(&graph);

    Some(find_worst_cost(&graph, &start, &end) as u32)
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
        assert_eq!(result, Some(94));
    }

    #[ignore]
    #[test]
    fn test_part_one_real() {
        let result = part_one(&aoc23_rust::template::read_file("inputs", DAY));
        assert_eq!(result, Some(2414));
    }

    #[ignore]
    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
