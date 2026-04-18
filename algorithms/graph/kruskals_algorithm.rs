// this is kruskals_algo that is used in this probelm to find min_cost from given points
// we just find out the MST and calculate cost 
// as it uses sorting of all edges that increase it TC from O(n^2) to O(n^2logn
//we use Prim's algorithm to reduce it to O(n^2)

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        
       
        let mut edges = Vec::new();
        
        for i in 0..n {
            for j in i + 1..n {
                let cost = (points[i][0] - points[j][0]).abs()
                         + (points[i][1] - points[j][1]).abs();
                edges.push((cost, i, j));
            }
        }

      
        edges.sort_by_key(|e| e.0);

      
        let mut parent: Vec<usize> = (0..n).collect();

        fn find(parent: &mut Vec<usize>, x: usize) -> usize {
            if parent[x] != x {
                parent[x] = find(parent, parent[x]); 
            }
            parent[x]
        }

        fn union(parent: &mut Vec<usize>, u: usize, v: usize) -> bool {
            let pu = find(parent, u);
            let pv = find(parent, v);

            if pu == pv {
                return false;
            }

            parent[pu] = pv;
            true
        }

       
        let mut answer = 0;
        let mut edges_used = 0;

        for (cost, u, v) in edges {
            if union(&mut parent, u, v) {
                answer += cost;
                edges_used += 1;

                if edges_used == n - 1 {
                    break;
                }
            }
        }

        answer
    }
}