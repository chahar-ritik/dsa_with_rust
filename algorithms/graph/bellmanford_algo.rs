//Bellmanford algorithm used when negative weight given in graph . Dijkstra algorithm fail with negative weight
//Bellmanford algo will not give shortest path for negative cyclic graph means total weight of edges in cycle gives negative then it will not give shortest path
fn main() {
    let edges = vec![
        //[u,v,w] u -> v , w is weight
        vec![3, 4, 4],
        vec![2, 3, 2],
        vec![1, 4, -1],
        vec![1, 2, -4],
        vec![0, 2, 4],
        vec![0, 1, 2],
    ];
    println!("Shortest path: {}", shortest_path(&edges, 5));

    fn shortest_path(edges: &Vec<Vec<i32>>, n: usize) -> i32 {
        let mut dist = vec![i32::MAX; n];

        dist[0] = 0;
        let mut updated = false;

        for i in 0..n - 1 {
            for edge in edges {
                let u = edge[0] as usize;
                let v = edge[1] as usize;
                let w = edge[2];
                if dist[u] != i32::MAX && dist[v] > dist[u] + w {
                    dist[v] = dist[u] + w;
                    updated = true;
                }
            }
            if !updated {
                break; // if there is no updates a value in whole inner loop early exit it will not update in any further iteration
            }
        }
        dist[n - 1]
    }
}

/*To detect negative cycle if it is possible to relax edge after v-1 times relaxtation of all edges that's a negative cycle 
// Step 2: Check for negative cycle
    for edge in edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        let w = edge[2];

        if dist[u] != i32::MAX && dist[v] > dist[u] + w {
            return true; // Negative cycle found
        }
    }

    false
 */