//topological sorting using bfs "kahn's algorithm" that use indegree of each vertices
//when indegree of any vertices zero push it to queue
//for topo sort graph must be DAG(Directed Acyclic Graph) as in DFS

use std::collections::VecDeque;
fn main(){
    let adj_list = vec![
        vec![],
        vec![],
        vec![3],
        vec![1],
        vec![0,1],
        vec![0,2]
    ];

    println!("{:?}" , topo_sort_bfs(&adj_list));
}


fn topo_sort_bfs(adj: &Vec<Vec<usize>>) -> Vec<usize>{
    let mut indeg = vec![0;adj.len()];
     let mut queue = VecDeque::new();
     let mut ans = Vec::new();
    for u in adj{
      for &v in u{
        indeg[v]+=1;
      }
    }

    for i in 0..indeg.len(){
      if indeg[i] == 0{
        queue.push_back(i);
      }
    }

    while let Some(vert) = queue.pop_front(){
        
        for &neighbor in &adj[vert]{
            indeg[neighbor] -=1;

            if indeg[neighbor] == 0{
                queue.push_back(neighbor);
            }
        }

           ans.push(vert);
    }
    ans
}