type Vertex = usize;
use std::collections::VecDeque;
use crate::graph_builder::Graph;

pub fn compute_distance_bfs(start: Vertex, graph: &Graph) -> Vec<Option<u32>>{
    let mut distance: Vec<Option<u32>> = vec![None;graph.n];
    distance[start] = Some(0); 
    let mut queue: VecDeque<Vertex> = VecDeque::new();
    queue.push_back(start);
    while let Some(v) = queue.pop_front() { 
        for u in graph.outedges[v].iter() {
            if let None = distance[*u] { 
                distance[*u] = Some(distance[v].unwrap() + 1);
                queue.push_back(*u);
            }
        }
    }
    distance
}
