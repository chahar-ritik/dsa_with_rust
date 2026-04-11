use std::collections::VecDeque;


//for use of graphs in real apps rust has libraries like petgraph
fn main() {
    // graph adjacency list (list of nodes where each node keeps a list of its neighbors)
    // using Vec<Vec<i32>>

    let mut graph: Vec<Vec<usize>> = vec![vec![1,3], vec![0, 2], vec![1], vec![0]];

    let mut visit_cyc = vec![false; graph.len()];
    println!("Is cycle (checked using DFS): {}", isCycle_undirect_dfs(0 , -1 ,  &graph , &mut visit_cyc ));
    println!("Is cycle (checked using BFS): {}", isCycle_undirect_bfs(0 ,  &graph ));


}

fn isCycle_undirect_dfs(node: usize , par: i32, adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>) -> bool {
    visited[node] = true;

    for &next in &adj[node] {
        if !visited[next]{
            if isCycle_undirect_dfs(next , node as i32 , adj , visited ){
                return true;
            }
        }else if next as i32 != par{
          return true;
        }
    }

    return false;
}


fn isCycle_undirect_bfs(node: usize , adj : &Vec<Vec<usize>>) -> bool{
    let mut visited = vec![false;adj.len()];
    let mut queue = VecDeque::new();

    visited[node] = true;
    queue.push_back((node , -1));

    while let Some((n , par)) = queue.pop_front(){
        for &neighbor in  &adj[n] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back((neighbor , n as i32))
            }else if neighbor as i32 != par {
               return true;
        }
        }
    }
    return false;

}
