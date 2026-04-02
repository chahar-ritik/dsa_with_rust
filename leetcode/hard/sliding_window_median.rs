use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let mut max_heap = BinaryHeap::new(); 
        let mut min_heap = BinaryHeap::new(); 
        let mut delayed = HashMap::new();     
        let mut res = Vec::new();

        let mut small_size = 0; 
        let mut large_size = 0; 

       
        let mut clean_max = |heap: &mut BinaryHeap<i32>, delayed: &mut HashMap<i32, i32>| {
            while let Some(&top) = heap.peek() {
                if let Some(cnt) = delayed.get_mut(&top) {
                    if *cnt > 0 {
                        *cnt -= 1;
                        heap.pop();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        };

    
        let mut clean_min = |heap: &mut BinaryHeap<Reverse<i32>>, delayed: &mut HashMap<i32, i32>| {
            while let Some(&Reverse(top)) = heap.peek() {
                if let Some(cnt) = delayed.get_mut(&top) {
                    if *cnt > 0 {
                        *cnt -= 1;
                        heap.pop();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        };

      
        let mut rebalance = |max_heap: &mut BinaryHeap<i32>,
                             min_heap: &mut BinaryHeap<Reverse<i32>>,
                             small_size: &mut i32,
                             large_size: &mut i32| {
            if *small_size > *large_size + 1 {
                let val = max_heap.pop().unwrap();
                min_heap.push(Reverse(val));
                *small_size -= 1;
                *large_size += 1;
            } else if *large_size > *small_size {
                let val = min_heap.pop().unwrap().0;
                max_heap.push(val);
                *large_size -= 1;
                *small_size += 1;
            }
        };

      
        let mut get_median = |max_heap: &BinaryHeap<i32>,
                              min_heap: &BinaryHeap<Reverse<i32>>,
                              k: i32| -> f64 {
            if k % 2 == 1 {
                *max_heap.peek().unwrap() as f64
            } else {
                (*max_heap.peek().unwrap() as f64
                    + min_heap.peek().unwrap().0 as f64) / 2.0
            }
        };

       
        for i in 0..k as usize {
            let num = nums[i];
            if max_heap.is_empty() || num <= *max_heap.peek().unwrap() {
                max_heap.push(num);
                small_size += 1;
            } else {
                min_heap.push(Reverse(num));
                large_size += 1;
            }
            rebalance(&mut max_heap, &mut min_heap, &mut small_size, &mut large_size);
        }

       
        res.push(get_median(&max_heap, &min_heap, k));

      
        for i in k as usize..nums.len() {
            let num = nums[i];
            let out = nums[i - k as usize];

       
            if num <= *max_heap.peek().unwrap() {
                max_heap.push(num);
                small_size += 1;
            } else {
                min_heap.push(Reverse(num));
                large_size += 1;
            }

         
            *delayed.entry(out).or_insert(0) += 1;

            if out <= *max_heap.peek().unwrap() {
                small_size -= 1;
            } else {
                large_size -= 1;
            }

          
            clean_max(&mut max_heap, &mut delayed);
            clean_min(&mut min_heap, &mut delayed);

          
            rebalance(&mut max_heap, &mut min_heap, &mut small_size, &mut large_size);

           
            clean_max(&mut max_heap, &mut delayed);
            clean_min(&mut min_heap, &mut delayed);

          
            res.push(get_median(&max_heap, &min_heap, k));
        }

        res
    }
}