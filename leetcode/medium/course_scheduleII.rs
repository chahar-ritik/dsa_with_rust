impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adj: Vec<Vec<usize>> = vec![vec![]; num_courses as usize];
        for edge in prerequisites {
            let u = edge[1];
            let v = edge[0];
            adj[u as usize].push(v as usize);
        }

        return order(&adj);

        fn order(adj: &Vec<Vec<usize>>) -> Vec<i32> {
            let num_vert = adj.len();

            let mut recpath = vec![false; num_vert];

            //topological sorting

            let mut visited = vec![false; num_vert];
            let mut stack = Vec::new();
            for vert in 0..num_vert {
                if !visited[vert] {
                    if dfs_cycle_helper(vert, adj, &mut visited, &mut recpath, &mut stack) {
                        return vec![];
                    }
                }
            }

            stack.reverse();
            stack
        }

        //dfs for cycle detecting and finding order
        fn dfs_cycle_helper(
            vert: usize,
            adj: &Vec<Vec<usize>>,
            visited: &mut Vec<bool>,
            recpath: &mut Vec<bool>,
            stack: &mut Vec<i32>,
        ) -> bool {
            visited[vert] = true;
            recpath[vert] = true;

            for &neighbor in &adj[vert] {
                if !visited[neighbor] {
                    if dfs_cycle_helper(neighbor, adj, visited, recpath, stack) {
                        return true;
                    }
                } else if recpath[neighbor] {
                    return true;
                }
            }

            recpath[vert] = false;
            stack.push(vert as i32);
            return false;
        }
    }
}
