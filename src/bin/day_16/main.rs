use std::collections::{HashMap, VecDeque};

use anyhow::{Context, Result};
use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;

#[derive(Eq, PartialEq, Debug)]
struct Problem {
    flow_rate: Vec<u32>,
    edges: Vec<Vec<usize>>,
    vertex_names: HashMap<String, usize>,
    costs: Vec<Vec<u32>>,
    vtx_with_flow: Vec<usize>,
}

fn parse(inp: &str) -> Result<Problem> {
    let mut flow_rate = Vec::new();
    let mut edges = Vec::new();
    let mut vertex_names = HashMap::new();
    let re =
        Regex::new(r"^Valve ([A-Z]+) has flow rate=([0-9]+); tunnels? leads? to valves? (.*)$")?;

    for (vtx, line) in inp.lines().filter(|line| !line.is_empty()).enumerate() {
        let caps = re.captures(line).context("Expected match")?;
        let name = caps.get(1).unwrap();
        vertex_names.insert(name.as_str().into(), vtx);
        let rate: u32 = caps.get(2).unwrap().as_str().parse()?;
        flow_rate.push(rate);
        edges.push(Vec::new());
    }

    for (vtx, line) in inp.lines().filter(|line| !line.is_empty()).enumerate() {
        let caps = re.captures(line).context("Expected match")?;
        let edge_names = caps.get(3).unwrap();
        for named_edge in edge_names.as_str().split(", ") {
            let idx = vertex_names
                .get(named_edge)
                .context("Undiscovered vertex")?;
            edges[vtx].push(*idx);
        }
    }
    let costs = shortest_paths(&edges, &flow_rate);

    let vtx_with_flow = flow_rate
        .iter()
        .cloned()
        .enumerate()
        .filter(|(_, flow)| *flow > 0)
        .map(|(vtx, _)| vtx)
        .collect();

    Ok(Problem {
        flow_rate,
        edges,
        vertex_names,
        costs,
        vtx_with_flow,
    })
}

fn shortest_paths(edges: &[Vec<usize>], flow_rate: &Vec<u32>) -> Vec<Vec<u32>> {
    let mut out = Vec::new();
    let mut queue: VecDeque<(u32, usize)> = VecDeque::new();
    for vtx in 0..flow_rate.len() {
        out.push(vec![0; flow_rate.len()]);
        for edge in edges[vtx].iter() {
            queue.push_back((1, *edge));
        }
        while !queue.is_empty() {
            let (dist, here) = queue.pop_front().unwrap();
            if out[vtx][here] == 0 {
                out[vtx][here] = dist;
                for edge in edges[here].iter() {
                    if *edge != vtx {
                        queue.push_back((dist + 1, *edge));
                    }
                }
            }
        }
    }
    out
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct State {
    loc: usize,
    cost: u32,
    flow: u32,
    released: u32,
    opened: u64,
}

fn search(problem: &Problem, max_cost: u32) -> u32 {
    let mut stack = Vec::new();
    let mut best = 0;

    let start_loc = *problem.vertex_names.get("AA").unwrap();

    for &loc in problem.vtx_with_flow.iter() {
        let cost = problem.costs[start_loc][loc];
        if cost == 0 {
            continue;
        }
        let flow = problem.flow_rate[loc];
        stack.push(State {
            loc,
            cost,
            flow: 0,
            released: 0,
            opened: 0,
        });
        if flow > 0 {
            stack.push(State {
                loc,
                cost: cost + 1,
                flow,
                released: 0,
                opened: 1 << (loc + 1),
            });
        }
    }
    stack = stack
        .iter()
        .cloned()
        .sorted_by_key(|state| state.flow)
        .collect();

    let max_flow: u32 = problem.flow_rate.iter().sum();

    while !stack.is_empty() {
        let mut pushed = false;
        let state = stack.pop().unwrap();
        let flow_here = problem.flow_rate[state.loc];
        let is_open = state.opened & (1 << (state.loc + 1)) > 0;
        if flow_here > 0 && !is_open && state.cost < max_cost + 1 {
            stack.push(State {
                loc: state.loc,
                cost: state.cost + 1,
                flow: state.flow + flow_here,
                released: state.released + state.flow,
                opened: state.opened | (1 << (state.loc + 1)),
            });
            pushed = true;
        }

        if state.cost < max_cost {
            let remaining = max_cost - state.cost;
            for vtx in problem.vtx_with_flow.iter() {
                let cost = problem.costs[state.loc][*vtx];
                let is_open = state.opened & (1 << (vtx + 1)) > 0;
                if *vtx != state.loc && cost + 1 < remaining && !is_open {
                    stack.push(State {
                        loc: *vtx,
                        cost: cost + state.cost,
                        flow: state.flow,
                        released: state.released + cost * state.flow,
                        opened: state.opened,
                    });
                    pushed = true;
                }
            }
        }

        if !pushed {
            let waiting_release = (max_cost - state.cost) * state.flow + state.released;
            if waiting_release > best {
                best = waiting_release;
            }
        }

        stack = stack
            .iter()
            .cloned()
            .filter(|state| best < state.released + (max_cost - state.cost) * max_flow)
            .collect();
    }

    best
}

fn search_2(problem: &Problem) -> u32 {
    let max_cost = 26;

    (1..(problem.vtx_with_flow.len() / 2 + 1))
        .into_par_iter()
        .map(|left_size| {
            problem
                .vtx_with_flow
                .iter()
                .combinations(left_size)
                .map(|left_set| {
                    let right_set = problem
                        .vtx_with_flow
                        .iter()
                        .cloned()
                        .filter(|vtx| !left_set.contains(&vtx))
                        .collect_vec();
                    let left = left_set.iter().map(|vtx| **vtx).collect_vec();
                    let right_prob = Problem {
                        flow_rate: problem.flow_rate.clone(),
                        edges: problem.edges.clone(),
                        vertex_names: problem.vertex_names.clone(),
                        costs: problem.costs.clone(),
                        vtx_with_flow: right_set,
                    };
                    let right_solution = search(&right_prob, max_cost);
                    let left_prob = Problem {
                        flow_rate: problem.flow_rate.clone(),
                        edges: problem.edges.clone(),
                        vertex_names: problem.vertex_names.clone(),
                        costs: problem.costs.clone(),
                        vtx_with_flow: left,
                    };
                    let left_solution = search(&left_prob, max_cost);
                    left_solution + right_solution
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::{parse, search};

    const EXAMPLE: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";
    #[test]
    fn test_parse_problem() {
        let problem = parse(EXAMPLE).unwrap();
        assert_eq!(problem.edges.len(), problem.vertex_names.len());
        assert_eq!(problem.flow_rate.len(), problem.vertex_names.len());
        assert_eq!(problem.vertex_names.get("AA"), Some(&0));
        println!("{problem:?}");
    }

    #[test]
    fn test_shortest_path() {
        let problem = parse(EXAMPLE).unwrap();
        assert_eq!(problem.costs[0][1], 1);
        assert_eq!(problem.costs[0][2], 2);
        assert_eq!(problem.costs[3][1], 2);
    }

    #[test]
    fn test_search() {
        let problem = parse(EXAMPLE).unwrap();
        let solution = search(&problem, 30);
        assert_eq!(solution, 1651);
    }

    #[test]
    fn test_search_2() {
        let problem = parse(EXAMPLE).unwrap();
        let solution = super::search_2(&problem);
        assert_eq!(solution, 1707);
    }
}

fn main() -> Result<()> {
    let buf = aoc::io::read_stdin()?;
    let problem = parse(buf.as_str())?;
    let cost = search(&problem, 30);
    println!("{cost}");
    let cost_2 = search_2(&problem);
    println!("{cost_2}");
    Ok(())
}
