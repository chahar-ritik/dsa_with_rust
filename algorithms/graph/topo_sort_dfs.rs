//topological sorting can be performed only on DAG (Directed Acyclic Graph)
//when we have any u -> v vertices u must come before v in the linear ordering like if there is to 2->1 , 4->1 order 2 and 4 must come before 1 use in problems like dependency, prerequisites

fn main(){
    let adj_list = vec![
        vec![],
        vec![],
        vec![3],
        vec![1],
        vec![0,1],
        vec![0,2]
    ];

    println!("{:?}" , topo_sort(&adj_list));
}

fn topo_sort(adj: &Vec<Vec<usize>>)-> Vec<usize>{
    let mut visited = vec![false;adj.len()];
    let mut stack = Vec::with_capacity(adj.len());

    for v in 0..adj.len(){
        if !visited[v]{
            dfs_helper(v, adj, &mut visited, &mut stack);
        }
    }

    stack.reverse();
    stack
}

fn dfs_helper(node: usize, adj: &Vec<Vec<usize>> , visited: &mut Vec<bool> , stack: &mut Vec<usize> ){
    visited[node] = true;
    

    for &next in &adj[node]{
        if !visited[next]{
            dfs_helper(next , adj , visited , stack)
        }
    }

    stack.push(node);
}