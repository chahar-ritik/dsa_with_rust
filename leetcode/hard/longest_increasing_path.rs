//LeetCode problem 329
// as it is asked for strictly increasing path that can't make cycle that's a DAG we can use Topo_sort wtih bfs
use std::collections::VecDeque;

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();

        let dirs = vec![(0,1),(0,-1),(1,0),(-1,0)];
        let mut indegree = vec![vec![0; n]; m];

      
        for i in 0..m {
            for j in 0..n {
                for (dx, dy) in &dirs {
                    let ni = i as i32 + dx;
                    let nj = j as i32 + dy;

                    if ni >= 0 && nj >= 0 && ni < m as i32 && nj < n as i32 {
                        let ni = ni as usize;
                        let nj = nj as usize;

                        if matrix[ni][nj] < matrix[i][j] {
                            indegree[i][j] += 1;
                        }
                    }
                }
            }
        }

       
        let mut q = VecDeque::new();
        for i in 0..m {
            for j in 0..n {
                if indegree[i][j] == 0 {
                    q.push_back((i, j));
                }
            }
        }

       
        let mut length = 0;

        while !q.is_empty() {
            let size = q.len();
            length += 1;

            for _ in 0..size {
                let (i, j) = q.pop_front().unwrap();

                for (dx, dy) in &dirs {
                    let ni = i as i32 + dx;
                    let nj = j as i32 + dy;

                    if ni >= 0 && nj >= 0 && ni < m as i32 && nj < n as i32 {
                        let ni = ni as usize;
                        let nj = nj as usize;

                        if matrix[ni][nj] > matrix[i][j] {
                            indegree[ni][nj] -= 1;

                            if indegree[ni][nj] == 0 {
                                q.push_back((ni, nj));
                            }
                        }
                    }
                }
            }
        }

        length
    }
}