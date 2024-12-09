use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use petgraph::{algo::has_path_connecting, dot::Dot, prelude::StableDiGraph};
use regex::Regex;

fn solve_a(valid_updates: &Vec<Vec<u64>>) -> u64 {
    valid_updates
        .into_iter()
        .map(|update| update[update.len() / 2])
        .sum()
}

fn solve_b(rules: &HashSet<(u64, u64)>, invalid_updates: Vec<Vec<u64>>) -> u64 {
    let mut graph: StableDiGraph<u64, (u64, u64)> = StableDiGraph::new();

    let node_values: HashSet<u64> = rules
        .iter()
        .flat_map(|&(p, s)| [p, s].into_iter())
        .collect();
    let mut nodes = HashMap::new();

    for node_value in node_values {
        let node_idx = graph.add_node(node_value);
        nodes.insert(node_value, node_idx);
    }

    for &(pred, succ) in rules {
        assert!(pred != succ);
        graph.add_edge(nodes[&succ], nodes[&pred], (pred, succ));
    }

    let mut sum = 0;

    // let mut space = DfsSpace::new(&graph);

    for mut update in invalid_updates {
        let mut clone_graph = graph.clone();
        let node_set: HashSet<u64> = update.iter().map(|v| *v).collect();
        clone_graph.retain_edges(|g, edge| {
            let (from, to) = g[edge];

            node_set.contains(&from) && node_set.contains(&to)
        });
        clone_graph.retain_nodes(|g, node| node_set.contains(&g[node]));

        drop(node_set);

        let dot = Dot::new(&clone_graph);
        std::fs::write("graph.dot", format!("{dot:?}")).unwrap();
        drop(dot);

        update.sort_by(|a, b| {
            let connect_a_b = has_path_connecting(&clone_graph, nodes[a], nodes[b], None);
            let connect_b_a = has_path_connecting(&clone_graph, nodes[b], nodes[a], None);
            match (connect_a_b, connect_b_a) {
                (false, false) => unreachable!(),
                (false, true) => Ordering::Greater,
                (true, false) => Ordering::Less,
                (true, true) => unreachable!("{a}, {b}"),
            }
        });

        sum += update[update.len() / 2];
    }

    sum
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut succ_rules: HashMap<u64, HashSet<u64>> = HashMap::new();
    let mut rules: HashSet<(u64, u64)> = HashSet::new();

    let rule_match = Regex::new(r"^(\d+)\|(\d+)$").unwrap();
    let update_match = Regex::new(r"^(\d+)(,(\d+))*$").unwrap();
    let update_parse = Regex::new(r"\d+").unwrap();

    let mut updates: Vec<Vec<u64>> = Vec::new();

    for line in input.lines() {
        if let Some(cap) = rule_match.captures(line) {
            let pred: u64 = cap[1].parse().unwrap();
            let succ: u64 = cap[2].parse().unwrap();

            succ_rules.entry(succ).or_default().insert(pred);
            rules.insert((pred, succ));
        }

        if update_match.is_match(line) {
            updates.push(
                update_parse
                    .find_iter(line)
                    .map(|mat| mat.as_str().parse::<u64>().unwrap())
                    .collect(),
            );
        }
    }

    let mut invalid_updates = Vec::new();
    let mut valid_updates = Vec::new();

    for update in updates {
        let mut is_valid = true;
        for pred_idx in 0..(update.len() - 1) {
            let pred = update[pred_idx];
            for succ_idx in pred_idx..update.len() {
                let succ = update[succ_idx];
                if let Some(rule) = succ_rules.get(&pred) {
                    if rule.contains(&succ) {
                        is_valid = false;
                        break;
                    }
                }
            }
            if !is_valid {
                break;
            }
        }

        if is_valid {
            valid_updates.push(update);
        } else {
            invalid_updates.push(update);
        }
    }

    let output_a = solve_a(&valid_updates);
    let output_b = solve_b(&rules, invalid_updates);

    println!("Task1: {output_a}");
    println!("Task1: {output_b}");
}
