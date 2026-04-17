use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    // edges: [u, v, w]
    let edges = vec![
        vec![0,1,2],
        vec![0,2,4],
        vec![1,2,1],
        vec![1,3,7],
        vec![2,4,3],
        vec![4,3,2],
        vec![3,5,1],
        vec![4,5,5]
    ];

    let n = 6;

    //build adjacency list
    let mut adj_list: Vec<Vec<(i32, i32)>> = vec![vec![]; n];

    for edge in edges {
        let u = edge[0];
        let v = edge[1];
        let w = edge[2];

        adj_list[u as usize].push((v, w));
    }

    let result = shortest_path(&adj_list, 0, 5);

    println!("Shortest distance from 0 to 5 = {}", result);
}

fn shortest_path(adj: &Vec<Vec<(i32, i32)>>, src: i32, dest: i32) -> i32 {
    let n = adj.len();

    let mut dist = vec![i32::MAX; n];
    let mut min_heap = BinaryHeap::new();

    dist[src as usize] = 0;
    min_heap.push(Reverse((0, src))); // (distance, node)

    while let Some(Reverse((curr_dist, node))) = min_heap.pop() {

     
        if curr_dist > dist[node as usize] {
            continue;
        }

      
        if node == dest {
            return curr_dist;
        }

        for &(neighbor, weight) in &adj[node as usize] {
            let new_dist = curr_dist + weight;

            if new_dist < dist[neighbor as usize] {
                dist[neighbor as usize] = new_dist;
                min_heap.push(Reverse((new_dist, neighbor)));
            }
        }
    }

    // if unreachable
    -1
}