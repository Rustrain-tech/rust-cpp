#[derive(Debug, Clone, Copy)]
struct Node {}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub cost: i64,
}
pub struct Graph {
    nodes: Vec<Node>,
    pub edges: Vec<Vec<Edge>>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Graph {
            nodes: vec![Node {}; n],
            edges: vec![vec![]; n],
        }
    }

    /// non-directed graph
    pub fn from_edges(n: usize, edges: &Vec<(usize, usize, i64)>) -> Self {
        let mut graph = Graph::new(n);
        for &(from, to, cost) in edges {
            graph.add_edge(from, to, cost);
            graph.add_edge(to, from, cost);
        }
        graph
    }

    /// directed graph
    pub fn from_directed_edges(n: usize, edges: &Vec<(usize, usize, i64)>) -> Self {
        let mut graph = Graph::new(n);
        for &(from, to, cost) in edges {
            graph.add_edge(from, to, cost);
        }
        graph
    }

    fn add_edge(&mut self, from: usize, to: usize, cost: i64) {
        self.edges[from].push(Edge { from, to, cost });
    }
}
