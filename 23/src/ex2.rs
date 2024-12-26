use crate::parse_input::{Computer, Connection};
use std::collections::{HashMap, HashSet};

const TLIMIT: f64 = 0.025;

type AdjacencyList = HashMap<Computer, HashSet<Computer>>;

struct SearchContext {
    current: Vec<Computer>,
    max_found: Vec<Computer>,
    total_steps: usize,
    steps: Vec<usize>,
    prev_steps: Vec<usize>,
}

impl SearchContext {
    fn new(vertex_count: usize) -> Self {
        Self {
            current: Vec::new(),
            max_found: Vec::new(),
            total_steps: 0,
            steps: vec![0; vertex_count],
            prev_steps: vec![0; vertex_count],
        }
    }
}

fn build_graph(input: &str) -> AdjacencyList {
    let mut adj: AdjacencyList = HashMap::new();
    for line in input.lines() {
        let Connection(a, b) = line.trim().parse().expect("Invalid connection format");
        adj.entry(a).or_default().insert(b);
        adj.entry(b).or_default().insert(a);
    }
    adj
}

fn are_connected(adj: &AdjacencyList, v1: &Computer, v2: &Computer) -> bool {
    adj.get(v1)
        .map(|neighbors| neighbors.contains(v2))
        .unwrap_or(false)
}

fn color_vertices(
    adj: &AdjacencyList,
    candidates: &mut [Computer],
    ctx: &SearchContext,
) -> Vec<usize> {
    let mut colors = vec![0; candidates.len()];
    let mut color_classes: Vec<Vec<Computer>> = vec![Vec::new()];
    let min_required_colors = ctx.max_found.len().saturating_sub(ctx.current.len()) + 1;

    let mut valid_count = 0;
    for i in 0..candidates.len() {
        let vertex = &candidates[i];
        let mut color = 0;
        while color < color_classes.len()
            && color_classes[color]
                .iter()
                .any(|&v| are_connected(adj, &v, vertex))
        {
            color += 1;
        }
        if color == color_classes.len() {
            color_classes.push(Vec::new());
        }
        color_classes[color].push(*vertex);

        if color < min_required_colors {
            candidates[valid_count] = *vertex;
            valid_count += 1;
        }
    }

    let mut idx = valid_count;
    for (color, class) in color_classes.iter().enumerate().skip(min_required_colors) {
        for &vertex in class {
            candidates[idx] = vertex;
            colors[idx] = color;
            idx += 1;
        }
    }
    colors
}

fn find_max_clique(adj: &AdjacencyList) -> Vec<Computer> {
    let mut candidates: Vec<_> = adj.keys().copied().collect();
    candidates.sort_by(|a, b| b.cmp(a));
    let mut ctx = SearchContext::new(candidates.len());
    find_max_clique_recursive(adj, &mut candidates, &mut ctx, 0);
    ctx.max_found
}

fn find_max_clique_recursive(
    adj: &AdjacencyList,
    candidates: &mut Vec<Computer>,
    ctx: &mut SearchContext,
    level: usize,
) {
    ctx.steps[level] =
        ctx.steps[level] + ctx.steps[level.saturating_sub(1)] - ctx.prev_steps[level];
    ctx.prev_steps[level] = ctx.steps[level.saturating_sub(1)];

    while !candidates.is_empty() {
        let colors = color_vertices(adj, candidates, ctx);
        let vertex = candidates.pop().expect("candidates not empty");

        if ctx.current.len() + colors[candidates.len()] <= ctx.max_found.len() {
            return;
        }

        ctx.current.push(vertex);
        let mut new_candidates: Vec<_> = candidates
            .iter()
            .filter(|&&v| are_connected(adj, &v, &vertex))
            .copied()
            .collect();

        if !new_candidates.is_empty() {
            if (ctx.steps[level] as f64) / (ctx.total_steps as f64) < TLIMIT {
                new_candidates.sort_by_cached_key(|&v| adj[&v].len());
            }
            ctx.steps[level] += 1;
            ctx.total_steps += 1;
            find_max_clique_recursive(adj, &mut new_candidates, ctx, level + 1);
        } else if ctx.current.len() > ctx.max_found.len() {
            ctx.max_found.clear();
            ctx.max_found.extend(ctx.current.iter().copied());
        }
        ctx.current.pop();
    }
}

pub fn solve(input: &str) -> String {
    let graph = build_graph(input);
    let mut res = find_max_clique(&graph)
        .iter()
        .map(Computer::to_string)
        .collect::<Vec<_>>();

    res.sort();
    res.join(",")
}
