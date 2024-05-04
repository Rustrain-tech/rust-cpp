use crate::lib::Graph::base::Edge;
use crate::lib::Graph::base::Graph;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn dijkstra(graph: &Graph, start: usize) -> Vec<i64> {
    let mut dist = vec![1 << 60; graph.edges.len()];
    let mut heap = BinaryHeap::new();
    dist[start] = 0;
    heap.push((Reverse(0), start));
    while let Some((Reverse(cost), node)) = heap.pop() {
        if dist[node] < cost {
            continue;
        }
        for &Edge { from, to, cost } in &graph.edges[node] {
            let new_cost = cost + dist[node];
            if new_cost < dist[to] {
                dist[to] = new_cost;
                heap.push((Reverse(new_cost), to));
            }
        }
    }
    dist
}
