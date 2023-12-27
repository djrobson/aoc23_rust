aoc23_rust::solution!(25);

use std::collections::HashMap;

use rustworkx_core::connectivity::stoer_wagner_min_cut;

use rustworkx_core::petgraph::graph::{NodeIndex, UnGraph};
use rustworkx_core::Result;


fn parse_input(input: &str) -> UnGraph<(), ()> {
    let mut vertices: HashMap<&str,NodeIndex> = HashMap::new();
    let mut graph : UnGraph<(), ()> = UnGraph::new_undirected();
    input
        .lines()
        .for_each(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            let first = parts[0];
            let rest = parts[1].trim().split(' ');
            let first_node: NodeIndex = 
                if !vertices.contains_key(first) {
                    let first_node: NodeIndex  = graph.add_node(());
                    vertices.insert(first, first_node);
                    first_node
                    
                } else {
                    *vertices.get(first).unwrap()
                };

            for rest_name in rest {
                let rest_node: NodeIndex = 
                    if !vertices.contains_key(rest_name) {
                        let rest_node = graph.add_node(());
                        vertices.insert(rest_name, rest_node);
                        rest_node
                    } else {
                        *vertices.get(rest_name).unwrap()
                    };
                graph.add_edge(first_node, rest_node, ());
            }
        });
    println!("found {} vertices", vertices.len());
    graph.shrink_to_fit();
    graph
}


pub fn part_one(input: &str) -> Option<u32> {
    let graph = parse_input(input);
    println!("{:?}", graph);
    let min_cut_res: Result<Option<(usize, Vec<_>)>> = stoer_wagner_min_cut(&graph, |_| Ok(1));

    let (min_cut, partition) = min_cut_res.unwrap().unwrap();

    assert_eq!(min_cut, 3);
    let (nodes, _) = graph.capacity();
    let part_size = partition.len() as u32;
    let cut_size = nodes as u32 - part_size;

    println!("part_size: {}, cut_size: {}", part_size, cut_size);
    Some(part_size * cut_size)
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
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
