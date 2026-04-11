use std::collections::VecDeque;
use std::collections::{HashMap,HashSet};

//for use of graphs in real apps rust has libraries like petgraph
fn main() {

 // graph adjacency list (list of nodes where each node keeps a list of its neighbors) 
 // using Vec<Vec<i32>> 

 let mut graph: Vec<Vec<usize>> =vec![
    vec![1,2,3],
    vec![0,3],
    vec![1,2],
    vec![0,3],
 ];

 let mut visited = vec![false ; graph.len()];
 println!("DFS");
 dfs(0, &graph , &mut visited );
 println!("BFS");
 bfs(0,&graph);

 //using HashMap as we have vertices as char
 let mut graph_with_hash: HashMap<char , Vec<char>> = HashMap::new();
   

graph_with_hash.insert('A',vec!['B','D']);
graph_with_hash.insert('B',vec!['A','C']);
graph_with_hash.insert('C',vec!['B']);
graph_with_hash.insert('D',vec!['A']);

let mut visit_hash = HashSet::new();
 println!("DFS");
 dfs_hash('A' , &graph_with_hash , &mut visit_hash);
 println!("BFS");
 bfs_hash('A' , &graph_with_hash);

}

fn dfs(node : usize , adj: &Vec<Vec<usize>> , visited: &mut Vec<bool>){
    visited[node] = true;
    println!("{}" , node);

    for &next in &adj[node]{
        if !visited[next]{
        dfs(next , adj , visited);
    }
}

}
fn bfs(start : usize , adj: &Vec<Vec<usize>>){
    let mut visited = vec![false ; adj.len()];
    let mut queue = VecDeque::new();

     queue.push_back(start);
     visited[start] = true;

     while let Some(node) = queue.pop_front(){
        println!("{}" , node);

        for &neighbor in &adj[node]{
            if !visited[neighbor]{
                 queue.push_back(neighbor);
                 visited[neighbor] = true;
            }
        }
     }
}

fn dfs_hash(node: char, adj: &HashMap<char, Vec<char>>, visited: &mut HashSet<char>) {
    if !visited.insert(node) {
        return; // already visited
    }

    println!("{}", node);

    if let Some(neighbors) = adj.get(&node) {
        for &neighbor in neighbors {
            if !visited.contains(&neighbor) {
                dfs_hash(neighbor, adj, visited);
            }
        }
    }
}

fn bfs_hash(node: char , adj :  &HashMap<char, Vec<char>>){
    let mut  visited = HashSet::new();
    let mut queue = VecDeque::new();

     queue.push_back(node);
     visited.insert(node);

     while let Some(n) = queue.pop_front(){
         println!("{}" , n);
          if let Some(neighbors) = adj.get(&n){
            for &neighbor in neighbors{
                if !visited.contains(&neighbor){
                queue.push_back(neighbor);
              visited.insert(neighbor);
                }
            }
          }
     }

}