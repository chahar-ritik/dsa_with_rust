use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct MedianFinder {
    min_heap : BinaryHeap<Reverse<i32>>,
    max_heap : BinaryHeap<i32>,
}


impl MedianFinder {

    fn new() -> Self {
        Self{
            min_heap: BinaryHeap::new(),
            max_heap: BinaryHeap::new(),
        }
    }
    
    fn add_num(&mut self, num: i32) {
        self.max_heap.push(num);

        let maxval = self.max_heap.pop().unwrap();

        self.min_heap.push(Reverse(maxval));

        if (self.min_heap.len() > self.max_heap.len() + 1) {
           let minval= (self.min_heap.pop().unwrap()).0;
         self.max_heap.push(minval);
        }
    }
    
    fn find_median(&self) -> f64 {
        if self.min_heap.len() > self.max_heap.len(){
            (*self.min_heap.peek().unwrap()).0 as f64
        }else{
           ((*self.max_heap.peek().unwrap() as f64
    + (*self.min_heap.peek().unwrap()).0 as f64) / 2.0)
        }
    }
}
