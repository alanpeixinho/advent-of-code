use std::{collections::{HashMap, HashSet}, io::stdin, iter::FromIterator};

fn read_input() -> (HashMap<usize, String>, HashSet<(usize, usize)>) {
    let named_edges: Vec<Vec<String>> = stdin().lines()
        .map(|line| line.expect("line error")
            .split('-')
            .map(|s| s.to_string())
            .collect())
        .collect();
    let mut inv_nodes = HashMap::new();
    for e in named_edges.iter() {
        for n in e {
            let next_id = inv_nodes.len();
            inv_nodes.entry(n.to_owned())
                .or_insert(next_id);
        }
    }

    let nodes: HashMap<usize, String> = HashMap::from_iter(inv_nodes.iter().map(|(k, &v)| (v, k.to_owned())));
    let edges = named_edges.into_iter()
     .map(|e| {
         let n1 = inv_nodes[&e[0]];
         let n2 = inv_nodes[&e[1]];
         if n1 < n2 { (n1, n2) }
         else { (n2, n1) }
     })
    .collect();
    (nodes, edges)
}

fn exists_edge(edges: &HashSet<(usize, usize)>, n1: usize, n2: usize) -> bool {
    if n1 < n2 {
        edges.contains(&(n1, n2))
    } else {
        edges.contains(&(n2, n1))
    }
}

fn is_clique3(edges: &HashSet<(usize, usize)>, nodes: (usize, usize, usize)) -> bool {
    exists_edge(&edges, nodes.0, nodes.1) && exists_edge(&edges, nodes.1, nodes.2) && exists_edge(&edges, nodes.0, nodes.2)
}

fn ordered(n1: usize, n2: usize, n3: usize) -> (usize, usize, usize) {
    let mut v = vec![n1, n2, n3];
    v.sort();
    (v[0], v[1], v[2])
}

fn neighbors(edges: &HashSet<(usize, usize)>, n: usize) -> HashSet<usize> {
    let n: HashSet<usize> = edges.iter()
        .filter_map(|&(n1, n2)|
            if n == n1 {
                Some(n2)
            } else if n == n2 {
                Some(n1)
            } else {
                None
            })
        .collect();
    n
}


//algorithm BronKerbosch1(R, P, X) is
    //if P and X are both empty then
        //report R as a maximal clique
    //for each vertex v in P do
        //BronKerbosch1(R ⋃ {v}, P ⋂ N(v), X ⋂ N(v))
        //P := P \ {v}
        //X := X ⋃ {v}
// probably an inneficient implementation, because I make several copies ... but it works
fn bron_kerbosch_rec(edges: &HashSet<(usize, usize)>,
    r: &mut HashSet<usize>, p: &mut HashSet<usize>, x: &mut HashSet<usize>,
    maximum: &mut HashSet<usize>) {
    if p.is_empty() && x.is_empty() {
        if r.len() > maximum.len() {
            maximum.clone_from(r);
        }
    }
    for v in p.clone() {
        let nv: HashSet<usize> = neighbors(edges, v);
        bron_kerbosch_rec(edges,
            &mut r.union(&HashSet::from([v])).map(|x| *x).collect(),
            &mut p.intersection(&nv).map(|x| *x).collect(),
            &mut x.intersection(&nv).map(|x| *x).collect(),
            maximum);
        p.remove(&v);
        x.remove(&v);
    }
}

fn maximum_clique(edges: &HashSet<(usize, usize)>, node_ids: &Vec<usize>) -> HashSet<usize> {
    let mut maximum = HashSet::with_capacity(node_ids.len());
    bron_kerbosch_rec(&edges,
        &mut HashSet::new(), &mut HashSet::from_iter(node_ids.iter().map(|x| *x)), &mut HashSet::new(),
        &mut maximum);
    maximum
}

pub fn main() {
    let (nodes, edges) = read_input();
    let t_nodes: HashSet<usize> = nodes.iter()
        .filter_map(|(&node, name)| if name.starts_with('t') { Some(node) } else { None })
        .collect();

    let node_ids: Vec<usize> = nodes.keys().map(|x| *x).collect();
    let mut all_clique3 = HashSet::new();
    for &n1 in t_nodes.iter() {
        for i in 0..nodes.len() {
            for j in (i+1)..nodes.len() {
                let n2 = node_ids[i];
                let n3 = node_ids[j];
                if n1 == n2 || n1 == n3 { continue; }
                let (n1, n2, n3) = ordered(n1, n2, n3);
                if is_clique3(&edges, (n1, n2, n3)) {
                    all_clique3.insert(ordered(n1, n2, n3));
                }
            }
        }
    }
    println!("Number of clique3 subgraphs: {:?}", all_clique3.len());

    let maximum: HashSet<usize> = maximum_clique(&edges, &node_ids);
    let mut maximum_names: Vec<String> = maximum.into_iter().map(|x| nodes[&x].clone()).collect();
    maximum_names.sort();
    println!("The maximum maximal clique is: {}", maximum_names.join(","));
}
