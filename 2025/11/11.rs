use std::{collections::{HashMap, HashSet}, io::stdin};

fn read_input() -> HashMap<String, HashSet<String>> {
    let mut adjacency = HashMap::<String, HashSet<String>>::new();
    for line in stdin().lines()
        .filter_map(|f| f.ok()) {
        let (node, neighbors) = line.split_once(':').expect("Not a valid line");
        adjacency.insert(node.to_owned(), neighbors.split(' ').skip(1).map(|s| s.to_owned()).collect());
    }
    adjacency
}

fn dfs(graph: &HashMap<String, HashSet<String>>,
    start: &str, end: &str,
    path: &mut Vec<String>, all_paths: &mut HashSet<Vec<String>>) {

    if start == end {
        all_paths.insert(path.clone());
        return;
    }
    for n in graph.get(start).unwrap_or(&HashSet::default()) {
        path.push(n.clone());
        dfs(graph, n, end, path, all_paths);
        path.pop();
    }
}

fn can_reach_dfs(graph: &HashMap<String, HashSet<String>>, start: &str, end: &str, visited: &mut HashSet<String>) -> bool {
    if start == end {
        return true;
    }
    visited.insert(start.to_owned());
    for n in graph.get(start).unwrap_or(&HashSet::default()) {
        if !visited.contains(n) && can_reach_dfs(graph, n, end, visited) {
            return true;
        }
    }
    false
}

// simplify thhe graph keeping only nodes that can reach some target
fn filter_reachable_nodes(graph: &HashMap<String, HashSet<String>>, target: &str) -> HashMap<String, HashSet<String>> {
    let ids: HashSet<String> = graph.keys()
        .filter(|node|
            can_reach_dfs(&graph, node, target, &mut HashSet::new()))
        .cloned().collect();

    let graph_filtered = graph.iter().filter(|(node, _)| ids.contains(*node))
        .map(|(node, adj)| (node.clone(), adj.clone())).collect();

    graph_filtered
}

fn count_paths(graph: &HashMap<String, HashSet<String>>, start: &str, target: &str) -> usize {
    // remove all nodes that cant reach our
    // target, for faster traverse
    // this is most likely a suboptimal solution
    // but this hack seems to be enough to run in a couple seconds
    let graph_pruned = filter_reachable_nodes(&graph, target);
    let mut paths_to_target = HashSet::new();
    dfs(&graph_pruned, start, target, &mut Vec::new(), &mut paths_to_target);
    paths_to_target.len()
}

pub fn main() {
    let graph = read_input();

    println!("# of paths: {}", count_paths(&graph, "you", "out"));

    //there are no paths from dac to fft, always from fft to dac
    println!("# of paths visiting dac and fft: {}",
        count_paths(&graph, "svr", "fft") *
        count_paths(&graph, "fft", "dac") *
        count_paths(&graph, "dac", "out"));
}
