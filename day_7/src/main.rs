use std::collections::{HashMap, HashSet};
use std::env;
use std::error;
use std::fs::File;
use std::io::prelude::*;

use daggy::Walker;

type Result<T> = std::result::Result<T, Box<error::Error>>;
type NodeIndex = daggy::NodeIndex;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = get_contents(filename)?;

    println!("{}", part_1(&contents)?);
    println!("{}", part_2(&contents)?);

    Ok(())
}

fn part_1(contents: &str) -> Result<String> {
    let (root_nodes, inv_node_mapping, graph) = get_roots_graph(contents)?;

    let result: String = get_part1_result(root_nodes, &graph)
        .iter()
        .map(|x| inv_node_mapping[x])
        .collect();

    Ok(result)
}

fn part_2(contents: &str) -> Result<usize> {
    let (root_nodes, _, graph) = get_roots_graph(contents)?;
    let nodes = root_nodes.clone();

    let result: usize = get_part2_result(nodes, &graph);

    Ok(result)
}

fn get_roots_graph(
    contents: &str,
) -> Result<(
    Vec<NodeIndex>,
    HashMap<NodeIndex, char>,
    daggy::Dag<char, char>,
)> {
    let (edges, nodes) = get_edges_nodes(contents)?;

    let mut nodes: Vec<char> = nodes.into_iter().collect();

    nodes.sort();

    let mut graph = daggy::Dag::<char, char>::new();
    let mut node_vec: Vec<NodeIndex> = Vec::new();
    let mut node_mapping: HashMap<char, NodeIndex> = HashMap::new();
    let mut inv_node_mapping: HashMap<NodeIndex, char> = HashMap::new();

    for node in nodes {
        let nodei = graph.add_node(node);
        node_vec.push(nodei);
        node_mapping.insert(node, nodei);
        inv_node_mapping.insert(nodei, node);
    }

    let edges: Vec<(NodeIndex, NodeIndex)> = edges
        .into_iter()
        .map(|x| (node_mapping[&x.0], node_mapping[&x.1]))
        .collect();

    graph.extend_with_edges(edges)?;

    let sorted_nodes: Vec<NodeIndex> = petgraph::algo::toposort(&graph, None)
        .unwrap()
        .into_iter()
        .collect();

    let root_nodes: Vec<NodeIndex> = get_roots(&sorted_nodes, &graph);

    Ok((root_nodes, inv_node_mapping, graph))
}

fn get_roots(nodes: &Vec<NodeIndex>, dag: &daggy::Dag<char, char>) -> Vec<NodeIndex> {
    let mut result: Vec<NodeIndex> = Vec::new();

    for node in nodes {
        let parents: Vec<NodeIndex> = dag.parents(*node).iter(&dag).map(|x| x.1).collect();
        if parents.len() == 0 {
            result.push(*node)
        }
    }

    result
}

fn get_children(node: NodeIndex, dag: &daggy::Dag<char, char>) -> Vec<NodeIndex> {
    let mut children: Vec<NodeIndex> = dag.children(node).iter(&dag).map(|x| x.1).collect();
    children.sort_by(|a, b| b.cmp(a));
    children
}

fn get_part1_result(
    nodes_to_consider: Vec<NodeIndex>,
    dag: &daggy::Dag<char, char>,
) -> Vec<NodeIndex> {
    let mut result: Vec<NodeIndex> = Vec::new();
    let mut nodes_to_consider = nodes_to_consider.clone();
    loop {
        let node = match nodes_to_consider.pop() {
            Some(x) => x,
            None => break,
        };
        update_nodes(&mut nodes_to_consider, &mut result, node, dag)
    }

    result
}

fn get_part2_result(mut nodes_to_consider: Vec<NodeIndex>, dag: &daggy::Dag<char, char>) -> usize {
    let mut workers: Vec<i64> = vec![-1; 4];
    let mut used: Vec<NodeIndex> = Vec::new();
    let mut node_times: HashMap<NodeIndex, usize> = HashMap::new();
    let mut time = 0;

    loop {
        let mut new_nodes_to_consider: Vec<NodeIndex> = Vec::new();
        let mut done_nodes: Vec<NodeIndex> = Vec::new();
        for windex in 0..3 {
            let node = match nodes_to_consider.pop() {
                Some(x) => x,
                None => break,
            };
            let node_entry = node_times.entry(node).or_insert(0);
            let index = node.index();
            if workers[windex] == -1 {
                workers[windex] = index as i64;
            }
            if workers[windex] == index as i64 {
                if *node_entry <= index {
                    *node_entry += 1;
                } else {
                    workers[windex] = -1;
                    done_nodes.push(node);
                }
                new_nodes_to_consider.push(node);
                println!(
                    "windex: {}, new_nodes_to_consider: {:?}",
                    windex, new_nodes_to_consider
                )
            } else {
                continue;
            }
        }
        println!("new nodes to consider: {:?}", new_nodes_to_consider);
        nodes_to_consider = new_nodes_to_consider;
        for node in done_nodes {
            update_nodes(&mut nodes_to_consider, &mut used, node, dag);
        }
        println!("nodes to consider: {:?}", nodes_to_consider);
        println!("workers: {:?}", workers);
        if nodes_to_consider.len() == 0 {
            break;
        }
        println!("{}: {:?}", time, node_times);
        if time == 16 {
            break;
        }
        time += 1;
    }

    time
}

fn update_nodes(
    nodes_to_consider: &mut Vec<NodeIndex>,
    used_nodes: &mut Vec<NodeIndex>,
    node: NodeIndex,
    dag: &daggy::Dag<char, char>,
) {
    used_nodes.push(node);
    let children = get_children(node, dag);
    for child in children {
        let parents: Vec<NodeIndex> = dag.parents(child).iter(&dag).map(|x| x.1).collect();
        let mut can_use = true;
        for parent in parents {
            if !used_nodes.contains(&parent) {
                can_use = false;
            }
        }
        if can_use {
            nodes_to_consider.push(child)
        }
    }
    nodes_to_consider.sort_by(|a, b| b.cmp(a));
}

fn get_contents(filename: &str) -> Result<String> {
    let mut f = File::open(filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok(contents)
}

fn get_edges_nodes(contents: &str) -> Result<(Vec<(char, char)>, HashSet<char>)> {
    let mut edges: Vec<(char, char)> = Vec::new();
    let mut nodes: HashSet<char> = HashSet::new();

    for line in contents.lines() {
        let e1 = line
            .chars()
            .nth(line.find("Step ").ok_or("can't find 'Step '!")? + 5)
            .ok_or("can't index string")?;
        let e2 = line
            .chars()
            .nth(line.find("step ").ok_or("can't find 'step '!")? + 5)
            .ok_or("can't index string")?;
        edges.push((e1, e2));
        nodes.insert(e1);
        nodes.insert(e2);
    }

    Ok((edges, nodes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() -> Result<()> {
        let contents = get_contents("test")?;
        assert!(part_1(&contents)? == "CABDFE");
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let contents = get_contents("test")?;
        assert!(part_2(&contents)? == 15);
        Ok(())
    }

}
