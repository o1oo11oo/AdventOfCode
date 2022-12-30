#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};
use petgraph::{
    algo::{astar, dijkstra},
    prelude::NodeIndex,
    visit::IntoNodeReferences,
    Graph, Undirected,
};

type AocGraph<'a> = Graph<(&'a str, u64), u64, Undirected, u32>;

pub(crate) fn part_1(input: &str) -> String {
    let g = parse_input(input).unwrap().1;
    let start = get_node_by_name(&g, "AA").unwrap();
    let time_remaining = 30;
    let top_amount = get_nodes_with_weight(&g).count().min(9);
    let released = find_max(g.clone(), top_amount, time_remaining, start, 0, vec![]);
    // let released = (
    //     1641,
    //     vec!["IZ", "CU", "QZ", "TU", "UZ", "FF", "GG", "ZL", "SY"],
    // );
    log::debug!("total for top {top_amount}: {released:?}");

    released.0.to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let g = parse_input(input).unwrap().1;
    let start = get_node_by_name(&g, "AA").unwrap();
    let time_remaining = 26;
    let nodes = g.node_references().map(|n| n.0).collect::<Vec<_>>();
    let value_nodes = get_nodes_with_weight(&g).collect::<Vec<_>>();
    let value_count = value_nodes.len();

    let mut paths = HashMap::new();
    for &start in &nodes {
        let start_name = get_node_name(&g, start).unwrap();
        let start_map = paths.entry(start).or_insert(HashMap::new());
        for &end in &nodes {
            let end_name = get_node_name(&g, end).unwrap();
            let res = astar(&g, start, |n| n == end, |e| *e.weight(), |_| 1).unwrap();
            start_map.insert(end, res);
        }
    }

    let mut score = 0;
    for top_amount in 1..=value_count {
        let released = find_max_with_elephant(
            g.clone(),
            top_amount,
            time_remaining,
            &paths,
            (start, None),
            (start, None),
            0,
            vec![],
        );
        log::info!("total for top {top_amount}: {released:?}");
        score = score.max(released.0);
    }

    score.to_string()
}

fn find_max<'a>(
    g: AocGraph<'a>,
    top_amount: usize,
    time_remaining: u64,
    start: NodeIndex,
    released: u64,
    node_list: Vec<&'a str>,
) -> (u64, Vec<&'a str>) {
    log::debug!(
        "time_remaining: {time_remaining}, total: {released}, selected: {}, list: {:?}",
        node_list.len(),
        node_list
    );

    if time_remaining == 0 || get_nodes_with_weight(&g).next().is_none() {
        log::debug!("finish: {released}, list: {:?}", node_list);
        return (released, node_list);
    }

    dijkstra(&g, start, None, |e| *e.weight())
        .iter()
        .filter_map(|(n, d)| {
            time_remaining
                .checked_sub(d + 1)
                .map(|r| (n, d, r, r * g.node_weight(*n).unwrap().1))
        })
        .sorted_by(|a, b| b.3.cmp(&a.3))
        .take(top_amount)
        .map(|(idx, dist, remaining, pressure_release)| {
            let mut g = g.clone();
            let mut node_list = node_list.clone();
            let (name, pressure) = g.node_weight_mut(*idx).unwrap();
            *pressure = 0;
            node_list.push(name);

            find_max(
                g,
                top_amount,
                remaining,
                *idx,
                released + pressure_release,
                node_list,
            )
        })
        .max_by(|a, b| a.0.cmp(&b.0))
        .unwrap_or((released, node_list))
}

#[allow(clippy::too_many_arguments)]
fn find_max_with_elephant<'a>(
    g: AocGraph<'a>,
    top_amount: usize,
    time_remaining: u64,
    paths: &HashMap<NodeIndex, HashMap<NodeIndex, (u64, Vec<NodeIndex>)>>,
    own: (NodeIndex, Option<VecDeque<NodeIndex>>),
    elephant: (NodeIndex, Option<VecDeque<NodeIndex>>),
    total_released: u64,
    node_list: Vec<&'a str>,
) -> (u64, Vec<&'a str>) {
    log::debug!("time_remaining: {time_remaining}, total: {total_released}, list: {node_list:?}");

    if time_remaining == 0 || get_nodes_with_weight(&g).next().is_none() {
        log::debug!("finish: {total_released}, list: {:?}", node_list);
        return (total_released, node_list);
    }

    let (own_pos, own_path) = own;
    let mut own_options = if let Some(own_path) = own_path {
        if let Some(&own_target) = own_path.back() {
            vec![(
                ((time_remaining - own_path.len() as u64) - 1)
                    * g.node_weight(own_target).unwrap().1,
                own_target,
                own_path,
            )]
        } else {
            let weight = g.node_weight(own_pos).unwrap().1;
            if weight == 0 {
                vec![(0, own_pos, VecDeque::from([own_pos]))]
            } else {
                vec![((time_remaining - 1) * weight, own_pos, own_path)]
            }
        }
    } else {
        paths
            .get(&own_pos)
            .unwrap()
            .iter()
            .filter_map(|(&target_idx, (dist, path))| {
                time_remaining.checked_sub(dist + 1).map(|remaining| {
                    (
                        remaining * g.node_weight(target_idx).unwrap().1,
                        target_idx,
                        VecDeque::from(path[1..].to_owned()),
                    )
                })
            })
            .filter(|n| n.0 > 0)
            .sorted_by(|a, b| b.0.cmp(&a.0))
            .collect::<Vec<_>>()
    };
    own_options.push((0, own_pos, VecDeque::from([own_pos])));
    log::debug!("own_options: {own_options:?}");

    let (ele_pos, ele_path) = elephant;
    let mut ele_options = if let Some(ele_path) = ele_path {
        if let Some(&ele_target) = ele_path.back() {
            vec![(
                ((time_remaining - ele_path.len() as u64) - 1)
                    * g.node_weight(ele_target).unwrap().1,
                ele_target,
                ele_path,
            )]
        } else {
            let weight = g.node_weight(ele_pos).unwrap().1;
            if weight == 0 {
                vec![(0, ele_pos, VecDeque::from([ele_pos]))]
            } else {
                vec![((time_remaining - 1) * weight, ele_pos, ele_path)]
            }
        }
    } else {
        paths
            .get(&ele_pos)
            .unwrap()
            .iter()
            .filter_map(|(&target_idx, (dist, path))| {
                time_remaining.checked_sub(dist + 1).map(|remaining| {
                    (
                        remaining * g.node_weight(target_idx).unwrap().1,
                        target_idx,
                        VecDeque::from(path[1..].to_owned()),
                    )
                })
            })
            .filter(|n| n.0 > 0)
            .sorted_by(|a, b| b.0.cmp(&a.0))
            .collect::<Vec<_>>()
    };
    ele_options.push((0, ele_pos, VecDeque::from([ele_pos])));
    log::debug!("ele_options: {ele_options:?}");

    let mut options = own_options
        .drain(..)
        .cartesian_product(ele_options)
        .filter(|(o, e)| o.1 != e.1 && o.0 + e.0 > 0)
        // check if switching is ok (probably only if pos are equal)
        .map(|(o, e)| {
            if own_pos == ele_pos && o.0 > e.0 {
                (o, e)
            } else {
                (e, o)
            }
        })
        .sorted_by(|a, b| (b.0 .0 + b.1 .0).cmp(&(a.0 .0 + a.1 .0)))
        .dedup()
        .take(top_amount)
        .collect::<Vec<_>>();
    log::debug!("options: {options:?}");

    options
        .drain(..)
        .map(
            |(
                (own_reduction, own_target, mut own_path),
                (ele_reduction, ele_target, mut ele_path),
            )| {
                let mut g = g.clone();
                let mut node_list = node_list.clone();
                let mut total_released = total_released;

                let own = if let Some(own_next) = own_path.pop_front() {
                    log::debug!(
                        "Self: {} -> {} (-> {})",
                        g.node_weight(own_pos).unwrap().0,
                        g.node_weight(own_next).unwrap().0,
                        g.node_weight(own_target).unwrap().0
                    );
                    (own_next, Some(own_path))
                } else {
                    let (name, weight) = g.node_weight_mut(own_target).unwrap();
                    log::debug!("Self: opening {name}");
                    *weight = 0;
                    total_released += own_reduction;
                    node_list.push(name);
                    (own_target, None)
                };

                let elephant = if let Some(ele_next) = ele_path.pop_front() {
                    log::debug!(
                        "Elephant: {} -> {} (-> {})",
                        g.node_weight(ele_pos).unwrap().0,
                        g.node_weight(ele_next).unwrap().0,
                        g.node_weight(ele_target).unwrap().0
                    );
                    (ele_next, Some(ele_path))
                } else {
                    let (name, weight) = g.node_weight_mut(ele_target).unwrap();
                    log::debug!("Elephant: opening {name}");
                    *weight = 0;
                    total_released += ele_reduction;
                    node_list.push(name);
                    (ele_target, None)
                };

                find_max_with_elephant(
                    g,
                    top_amount,
                    time_remaining - 1,
                    paths,
                    own,
                    elephant,
                    total_released,
                    node_list,
                )
            },
        )
        .max_by(|a, b| a.0.cmp(&b.0))
        .unwrap_or((total_released, node_list))
}

#[allow(clippy::too_many_arguments)]
fn find_max_with_elephant_old<'a>(
    g: AocGraph<'a>,
    top_amount: usize,
    time_remaining: u64,
    own_pos: NodeIndex,
    mut own_target: Option<NodeIndex>,
    elephant_pos: NodeIndex,
    mut elephant_target: Option<NodeIndex>,
    mut released: u64,
    node_list: Vec<&'a str>,
) -> (u64, Vec<&'a str>) {
    log::info!(
        "time_remaining: {time_remaining}, total: {released}, selected: {}, list: {:?}",
        node_list.len(),
        node_list
    );

    if time_remaining == 0 || get_nodes_with_weight(&g).next().is_none() {
        log::info!("finish: {released}, list: {:?}", node_list);
        return (released, node_list);
    }

    let own_targets = if let Some(tgt) = own_target {
        let (d, _) = astar(&g, own_pos, |n| n == tgt, |e| *e.weight(), |_| 1).unwrap();
        time_remaining
            .checked_sub(d + 1)
            .map(|r| {
                vec![(
                    tgt,
                    g.node_weight(tgt).unwrap().0,
                    d,
                    r,
                    r * g.node_weight(tgt).unwrap().1,
                )]
            })
            .unwrap()
    } else {
        let own_res = dijkstra(&g, own_pos, None, |e| *e.weight());
        own_res
            .iter()
            .chain(std::iter::once((&own_pos, &0)))
            .filter_map(|(n, d)| {
                time_remaining.checked_sub(d + 1).map(|r| {
                    (
                        *n,
                        g.node_weight(*n).unwrap().0,
                        *d,
                        r,
                        r * g.node_weight(*n).unwrap().1,
                    )
                })
            })
            .sorted_by(|a, b| b.4.cmp(&a.4))
            .collect::<Vec<_>>()
    };

    let elephant_targets = if let Some(tgt) = elephant_target {
        let (d, _) = astar(&g, elephant_pos, |n| n == tgt, |e| *e.weight(), |_| 1).unwrap();
        time_remaining
            .checked_sub(d + 1)
            .map(|r| {
                vec![(
                    tgt,
                    g.node_weight(tgt).unwrap().0,
                    d,
                    r,
                    r * g.node_weight(tgt).unwrap().1,
                )]
            })
            .unwrap()
    } else {
        let elephant_res = dijkstra(&g, elephant_pos, None, |e| *e.weight());
        elephant_res
            .iter()
            .chain(std::iter::once((&elephant_pos, &0)))
            .filter_map(|(n, d)| {
                time_remaining.checked_sub(d + 1).map(|r| {
                    (
                        *n,
                        g.node_weight(*n).unwrap().0,
                        *d,
                        r,
                        r * g.node_weight(*n).unwrap().1,
                    )
                })
            })
            .sorted_by(|a, b| b.4.cmp(&a.4))
            .collect::<Vec<_>>()
    };

    own_targets
        .iter()
        .cartesian_product(elephant_targets.iter())
        .map(|(&a, &b)| (a.min(b), a.max(b)))
        .filter(|&(a, b)| a.0 != b.0)
        .sorted_by(|&a, &b| (b.0 .4 + b.1 .4).cmp(&(a.0 .4 + a.1 .4)))
        .dedup()
        .take(top_amount)
        .map(|(own, elephant)| {
            log::info!("{:?}", (own, elephant));
            let mut g = g.clone();
            let mut node_list = node_list.clone();

            let (own_target_idx, own_name, own_dist, own_remaining, own_pressure_release) = own;
            let own_next = if g.node_weight(own_target_idx).unwrap().1 == 0 {
                own_pos
            } else if own_pos == own_target_idx {
                let (name, pressure) = g.node_weight_mut(own_target_idx).unwrap();
                *pressure = 0;
                node_list.push(name);
                released += own_pressure_release;
                own_target = None;
                log::info!("You open {name}");

                own_pos
            } else {
                own_target = Some(own_target_idx);
                astar(&g, own_pos, |n| n == own_target_idx, |e| *e.weight(), |_| 1)
                    .unwrap()
                    .1[1]
            };

            let (
                elephant_target_idx,
                elephant_name,
                elephant_dist,
                elephant_remaining,
                elephant_pressure_release,
            ) = elephant;
            let elephant_next = if g.node_weight(elephant_target_idx).unwrap().1 == 0 {
                elephant_pos
            } else if elephant_pos == elephant_target_idx {
                let (name, pressure) = g.node_weight_mut(elephant_target_idx).unwrap();
                *pressure = 0;
                node_list.push(name);
                released += elephant_pressure_release;
                elephant_target = None;
                log::info!("Elephant opens {name}");

                elephant_pos
            } else {
                elephant_target = Some(elephant_target_idx);
                astar(
                    &g,
                    elephant_pos,
                    |n| n == elephant_target_idx,
                    |e| *e.weight(),
                    |_| 1,
                )
                .unwrap()
                .1[1]
            };

            log::info!(
                "own_next: {own_next:?} {}",
                g.node_weight(own_next).unwrap().0
            );
            log::info!(
                "elephant_next: {elephant_next:?} {}",
                g.node_weight(elephant_next).unwrap().0
            );

            find_max_with_elephant_old(
                g,
                top_amount,
                time_remaining - 1,
                own_next,
                own_target,
                elephant_next,
                elephant_target,
                released,
                node_list,
            )
        })
        .max_by(|a, b| a.0.cmp(&b.0))
        .unwrap_or((released, node_list))
}

fn get_node_name<'a>(g: &'a AocGraph, idx: NodeIndex) -> Option<&'a str> {
    g.node_weight(idx).map(|n| n.0)
}

fn get_node_by_name(g: &AocGraph, name: &str) -> Option<NodeIndex> {
    g.node_references()
        .find(|(_, (n, _))| *n == name)
        .map(|n| n.0)
}

fn get_node_by_weight(g: &AocGraph, weight: u64, ignore_name: &str) -> Option<NodeIndex> {
    g.node_references()
        .find(|(_, (n, w))| *w == weight && *n != ignore_name)
        .map(|n| n.0)
}

fn get_nodes_with_weight<'a>(g: &'a AocGraph) -> impl Iterator<Item = NodeIndex> + 'a {
    g.node_references()
        .filter(|(_, (_, w))| *w > 0)
        .map(|n| n.0)
}

fn simplify_graph(g: AocGraph) -> AocGraph {
    while let Some(idx) = get_node_by_weight(&g, 0, "AA") {
        let neighbours = g.edges(idx);
        for n in neighbours {
            //g.remove_edge(n.);
        }
    }

    g
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
