use std::collections::HashMap;

use libadvent::*;

type Graph = HashMap<String, Vec<String>>;
type Path = Vec<String>;
type Paths = Vec<Path>;

const START_NODE: &str = "start";
const END_NODE: &str = "end";

fn is_lowercase(c: &char) -> bool {
    return ('a'..'z').contains(c);
}

fn is_uppercase(c: &char) -> bool {
    return ('A'..'Z').contains(c);
}

fn dfs(graph: &Graph, current_path: Path, part2: bool) -> Paths {
    let current_node = current_path.last().map(|s| s.as_str()).unwrap_or(START_NODE);

    if current_node == END_NODE {
        return vec![current_path];
    }

    let potential_nodes = graph.get(current_node).unwrap().iter().filter(|&node| {
        if node == START_NODE {
            return false;
        }

        if node == END_NODE {
            return true;
        }

        let is_big_node = is_uppercase(&node.chars().next().unwrap());
        let is_visited = current_path.contains(node);

        let num_small_nodes_visited_twice = count_occurences(current_path.iter().filter(|n| {
            is_lowercase(&n.chars().next().unwrap())
        })).into_iter().filter(|(_, c)| *c > 1).count();

        return is_big_node || !is_visited || (num_small_nodes_visited_twice == 0 && part2);
    });

    let mut paths = Vec::new();
    for node in potential_nodes {
        let mut path = current_path.clone();
        path.push(node.clone());

        for path in dfs(graph, path, part2) {
            paths.push(path);
        }
    }

    return paths;
}

fn main() {
    let input = must_read_input_to_lines();

    let mut graph: Graph = HashMap::new();

    for line in input.iter() {
        let (from, to) = line.split_once("-").unwrap();
        
        let mut from_edges = graph.remove(from).unwrap_or(Vec::new());
        from_edges.push(to.to_owned());
        graph.insert(from.to_owned(), from_edges);

        let mut to_edges = graph.remove(to).unwrap_or(Vec::new());
        to_edges.push(from.to_owned());
        graph.insert(to.to_owned(), to_edges);
    }

    println!("{:?}", dfs(&graph, vec![], false).len());
    println!("{:?}", dfs(&graph, vec![], true).len());
}
