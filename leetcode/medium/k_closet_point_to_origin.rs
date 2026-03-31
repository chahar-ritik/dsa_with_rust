use std::collections::BinaryHeap;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut max_heap = BinaryHeap::new();
        

        for point in points{
            let dist = point[0]*point[0] + point[1]*point[1];
             max_heap.push((dist , point));
            if max_heap.len() > k as usize{
                max_heap.pop();
               
            }
       }

       let res: Vec<Vec<i32>> = max_heap
    .into_iter()
    .map(|(_, point)| point)
    .collect();

       res
    }
}