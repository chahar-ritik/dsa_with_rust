use std::cmp::Reverse;
use std::collections::{HashMap, BinaryHeap};

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut freq_map = HashMap::new();

       
        for word in words {
            *freq_map.entry(word).or_insert(0) += 1;
        }

        let mut heap = BinaryHeap::new();

        for (word, freq) in freq_map.into_iter() {
            // include lex order in comparator
            heap.push(Reverse((freq, Reverse(word))));

           
            if heap.len() > k as usize {
                heap.pop();
            }
        }

       
        let mut res = Vec::with_capacity(k as usize);

        while let Some(Reverse((_, Reverse(word)))) = heap.pop() {
            res.push(word);
        }

        res.reverse();
        res
    }
}