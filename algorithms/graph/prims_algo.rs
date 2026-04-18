//It is not the standard form of Prim's algorithm that uses min_heap for minimum cost neighbor 
//Instead we use a vec that stores min_distance for each edge and update for neigbours when get less 

use std::collections::BinaryHeap;
use std::cmp::Reverse;
impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut visited = vec![false;n];
        let mut min_dist = vec![i32::MAX;n];
        let mut total = 0;

        min_dist[0] = 0;
       for _ in 0..n{
        let mut u:i32 = -1 ;
          for i in 0..n{
            if !visited[i] && ( u == -1 || min_dist[i] < min_dist[u as usize]){
                u = i as i32;
            }
          }
           
           let u = u as usize;
         visited[u] = true;
           total+=min_dist[u];
         for v in 0..n{
           let dist = (points[u][0] - points[v][0]).abs() + (points[u][1] - points[v][1]).abs();
           if dist < min_dist[v]{
            min_dist[v]  = dist;
           }
         }
        }
           total
       }
    }
