//for finding max sum of subsets of an array
// most efficient approach

use std::cmp;


fn main(){
    let a = [1,4,-6,7,-8,9,2 ];
    let maxsum_of_arr = max_subsset(&a);
    println!("{}",maxsum_of_arr);

}

fn max_subsset(a:&[i32]) -> i32{
   let mut currsum = 0;
   let mut maxsum = i32::MIN;
   for &i in a{
    currsum += i;
    maxsum = cmp::max(currsum , maxsum);
    if currsum < 0{
        currsum = 0;
    }

   } 

  maxsum


}