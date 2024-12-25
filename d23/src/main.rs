use std::collections::{HashMap, HashSet};

use petgraph::{
    graph::{NodeIndex, UnGraph},
    visit::{GetAdjacencyMatrix, IntoNeighbors, IntoNodeIdentifiers},
};

use std::hash::Hash;

/// ----------------------------- CODE START from https://github.com/petgraph/petgraph/pull/662 -----------------------------
/// Finds maximal cliques containing all the vertices in r, some of the
/// vertices in p, and none of the vertices in x.
fn bron_kerbosch_pivot<G>(
    g: G,
    adj_mat: &G::AdjMatrix,
    r: HashSet<G::NodeId>,
    mut p: HashSet<G::NodeId>,
    mut x: HashSet<G::NodeId>,
) -> Vec<HashSet<G::NodeId>>
where
    G: GetAdjacencyMatrix + IntoNeighbors,
    G::NodeId: Eq + Hash,
{
    let mut cliques = Vec::with_capacity(1);
    if p.is_empty() {
        if x.is_empty() {
            cliques.push(r);
        }
        return cliques;
    }
    // pick the pivot u to be the vertex with max degree
    let u = p.iter().max_by_key(|&v| g.neighbors(*v).count()).unwrap();
    let mut todo = p
        .iter()
        .filter(|&v| *u == *v || !g.is_adjacent(adj_mat, *u, *v) || !g.is_adjacent(adj_mat, *v, *u)) //skip neighbors of pivot
        .cloned()
        .collect::<Vec<G::NodeId>>();
    while let Some(v) = todo.pop() {
        let neighbors = HashSet::from_iter(g.neighbors(v));
        p.remove(&v);
        let mut next_r = r.clone();
        next_r.insert(v);

        let next_p = p
            .intersection(&neighbors)
            .cloned()
            .collect::<HashSet<G::NodeId>>();
        let next_x = x
            .intersection(&neighbors)
            .cloned()
            .collect::<HashSet<G::NodeId>>();

        cliques.extend(bron_kerbosch_pivot(g, adj_mat, next_r, next_p, next_x));

        x.insert(v);
    }

    cliques
}

/// Find all maximal cliques in a graph using Bron–Kerbosch algorithm
/// with pivoting.
///
/// A clique is a set of nodes such that every node connects to
/// every other. A maximal clique is a clique that cannot be extended
/// by including one more adjacent vertex. A graph may have multiple
/// maximal cliques.
///
/// Example
/// ```
/// use petgraph::algo::maximal_cliques;
/// use petgraph::graph::UnGraph;
/// use std::collections::HashSet;
///
/// let mut g = UnGraph::<i32, ()>::from_edges(&[(0, 1), (0, 2), (1, 2), (2, 3)]);
/// g.add_node(4);
/// // The example graph:
/// //
/// // 0 --- 2 -- 3
/// //  \   /
/// //   \ /
/// //    1       4
/// //
/// // maximal cliques: {4}, {2, 3}, {0, 1, 2}
/// // Output the result
/// let cliques = maximal_cliques(&g);
/// println!("{:?}", cliques);
/// // [
/// //   {NodeIndex(4)},
/// //   {NodeIndex(0), NodeIndex(1), NodeIndex(2)},
/// //   {NodeIndex(2), NodeIndex(3)}
/// // ]
/// ```
pub fn maximal_cliques<G>(g: G) -> Vec<HashSet<G::NodeId>>
where
    G: GetAdjacencyMatrix + IntoNodeIdentifiers + IntoNeighbors,
    G::NodeId: Eq + Hash,
{
    let adj_mat = g.adjacency_matrix();
    let r = HashSet::new();
    let p = g.node_identifiers().collect::<HashSet<G::NodeId>>();
    let x = HashSet::new();
    return bron_kerbosch_pivot(g, &adj_mat, r, p, x);
}
/// ----------------------------- CODE END from https://github.com/petgraph/petgraph/pull/662 -----------------------------

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    // let input = std::fs::read_to_string("example.txt").unwrap();

    let connections: Vec<(&str, &str)> = input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .collect();

    let mut graph = UnGraph::<&str, ()>::new_undirected();

    let mut node_mapping = HashMap::new();

    for (a, b) in connections {
        let node_a = *node_mapping.entry(a).or_insert_with(|| graph.add_node(a));
        let node_b = *node_mapping.entry(b).or_insert_with(|| graph.add_node(b));
        graph.add_edge(node_a, node_b, ());
    }

    let mut triangles = 0;
    for a in graph.node_indices() {
        let a_contains_t = graph.node_weight(a).unwrap().starts_with('t');
        for b in graph.neighbors(a) {
            if b > a {
                // To avoid duplicate checks
                let b_contains_t = graph.node_weight(b).unwrap().starts_with('t');
                for c in graph.neighbors(b) {
                    if c > b && graph.contains_edge(c, a) {
                        let c_contains_t = graph.node_weight(c).unwrap().starts_with('t');
                        if a_contains_t || b_contains_t || c_contains_t {
                            triangles += 1;
                        }
                    }
                }
            }
        }
    }

    let max_clique = maximal_cliques(&graph)
        .into_iter()
        .max_by_key(|clique| clique.len())
        .unwrap();
    let max_clique = max_clique.into_iter().collect::<Vec<NodeIndex>>();
    let mut max_clique_members: Vec<&str> = max_clique
        .into_iter()
        .map(|node| *graph.node_weight(node).unwrap())
        .collect();
    max_clique_members.sort();
    let max_clique_pw = max_clique_members.join(",");

    println!("Task1: {triangles}");
    println!("Task2: {max_clique_pw}");
}
