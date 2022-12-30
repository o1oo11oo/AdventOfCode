use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};
use petgraph::{algo::dijkstra, prelude::NodeIndex, visit::IntoNodeReferences, Graph, Undirected};

type AocGraph<'a> = Graph<(&'a str, u64), u64, Undirected, u32>;

pub(crate) fn part_1(input: &str) -> String {
    let g = parse_input(input).unwrap().1;
    let start = get_node_by_name(&g, "AA").unwrap();
    let paths = get_paths_from_graph(&g);
    let value_nodes = get_nodes_with_weight(&g).collect::<Vec<_>>();
    let time_remaining = 30;

    let max = value_nodes.iter().map(|n| n.index()).max().unwrap();
    let mut weights = vec![("", 0); max + 1];
    for node in &value_nodes {
        weights[node.index()] = *g.node_weight(*node).unwrap();
    }

    // best: ["IZ", "CU", "QZ", "TU", "UZ", "FF", "GG", "ZL", "SY"] with 1641
    let mut cache = HashMap::new();
    let score = calculate_score(
        &value_nodes,
        &weights,
        &paths,
        start,
        time_remaining,
        vec![],
        &mut cache,
    );

    score.to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let g = parse_input(input).unwrap().1;
    let start = get_node_by_name(&g, "AA").unwrap();
    let paths = get_paths_from_graph(&g);
    let value_nodes = get_nodes_with_weight(&g).collect::<Vec<_>>();
    let time_remaining = 26;

    let max = value_nodes.iter().map(|n| n.index()).max().unwrap();
    let mut weights = vec![("", 0); max + 1];
    for node in &value_nodes {
        weights[node.index()] = *g.node_weight(*node).unwrap();
    }

    let possibilities = 2u64.pow(value_nodes.len() as u32 - 1);
    let mut score = 0;

    // own_best: ["JH", "OI", "GG", "ZL", "XF", "TR", "SZ", "FF"] (not in this order)
    // ele_best: ["QZ", "TU", "IZ", "YL", "UZ", "PA", "CU"] (not in this order)
    for (own_nodes, ele_nodes) in
        (0..possibilities).map(|selection| select_elements(&value_nodes, selection))
    {
        let mut cache = HashMap::new();
        let own_score = calculate_score(
            &own_nodes,
            &weights,
            &paths,
            start,
            time_remaining,
            vec![],
            &mut cache,
        );

        let ele_score = calculate_score(
            &ele_nodes,
            &weights,
            &paths,
            start,
            time_remaining,
            vec![],
            &mut cache,
        );

        score = score.max(own_score + ele_score);
    }

    score.to_string()
}

fn select_elements(elements: &[NodeIndex], mut selection: u64) -> (Vec<NodeIndex>, Vec<NodeIndex>) {
    let mut own = vec![];
    let mut ele = vec![];
    for &node in elements {
        if selection % 2 == 0 {
            own.push(node);
        } else {
            ele.push(node);
        }
        selection /= 2;
    }
    (own, ele)
}

fn calculate_score(
    possible_nodes: &[NodeIndex],
    weights: &[(&str, u64)],
    paths: &HashMap<NodeIndex, HashMap<NodeIndex, u64>>,
    position: NodeIndex,
    time_left: u64,
    open_list: Vec<NodeIndex>,
    cache: &mut HashMap<(NodeIndex, u64, Vec<NodeIndex>), u64>,
) -> u64 {
    let position_paths = paths.get(&position).unwrap();
    possible_nodes
        .iter()
        .filter(|n| !open_list.contains(n))
        .filter_map(|&node| {
            let distance = position_paths.get(&node).unwrap();
            time_left
                .checked_sub(distance + 1)
                .map(|time_left| (node, time_left))
        })
        .map(|(node, time_left)| {
            let mut open_list = open_list.clone();
            open_list.push(node);
            open_list.sort();
            let key = (node, time_left, open_list.clone());
            let possible_score_remaining = if let Some(&possible_score_remaining) = cache.get(&key)
            {
                possible_score_remaining
            } else {
                let value = calculate_score(
                    possible_nodes,
                    weights,
                    paths,
                    node,
                    time_left,
                    open_list,
                    cache,
                );
                cache.insert(key, value);
                value
            };

            time_left * weights[node.index()].1 + possible_score_remaining
        })
        .max()
        .unwrap_or(0)
}

fn get_node_by_name(g: &AocGraph, name: &str) -> Option<NodeIndex> {
    g.node_references()
        .find(|(_, (n, _))| *n == name)
        .map(|n| n.0)
}

fn get_nodes_with_weight<'a>(g: &'a AocGraph) -> impl Iterator<Item = NodeIndex> + 'a {
    g.node_references()
        .filter(|(_, (_, w))| *w > 0)
        .map(|n| n.0)
}

fn get_paths_from_graph(g: &AocGraph) -> HashMap<NodeIndex, HashMap<NodeIndex, u64>> {
    let mut paths = HashMap::new();
    for start in g.node_indices() {
        paths.insert(start, dijkstra(&g, start, None, |e| *e.weight()));
    }
    paths
}

fn parse_input(input: &str) -> IResult<&str, AocGraph> {
    let mut g = Graph::new_undirected();

    let (input, nodes) = separated_list1(
        line_ending,
        tuple((
            preceded(tag("Valve "), alpha1),
            preceded(tag(" has flow rate="), nom::character::complete::u64),
            preceded(
                alt((
                    tag("; tunnels lead to valves "),
                    tag("; tunnel leads to valve "),
                )),
                separated_list1(tag(", "), alpha1),
            ),
        )),
    )(input)?;

    for &(name, flow, _) in &nodes {
        g.add_node((name, flow));
    }
    for (source_name, _, targets) in nodes {
        for target_name in targets {
            let source_index = get_node_by_name(&g, source_name).unwrap();
            let target_index = get_node_by_name(&g, target_name).unwrap();
            if g.find_edge(source_index, target_index).is_none() {
                g.add_edge(
                    source_index.min(target_index),
                    source_index.max(target_index),
                    1,
                );
            }
        }
    }

    Ok((input, g))
}
