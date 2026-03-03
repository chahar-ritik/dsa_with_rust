use std::collections::VecDeque;
impl Solution {
    pub fn time_required_to_buy(mut tickets: Vec<i32>, k: i32) -> i32 {
        let mut queue = VecDeque::from(tickets);
        let mut idx = k as usize;
        let mut time = 0;
         while queue[idx] != 0{
            let mut ticket = queue.pop_front().unwrap();
            time+=1;
            if ticket > 1_i32 || idx == 0{
                ticket-=1;
                queue.push_back(ticket);
            }
            if idx == 0{
                
                if queue.len() == 0{
                    break;
                }
                idx = queue.len().saturating_sub(1);
            }else{
                idx-=1;
            }
         } 
         time
    }
}