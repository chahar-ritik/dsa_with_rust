impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
      let mut st = 0;
      let mut end = arr.len()-1; // only for peak mountain value not in simple BS


      while(st < end){
        let mid = st + (end -st)/2;
       /*  if arr[mid-1] < arr[mid] && arr[mid+1] < arr[mid]{
            return mid as i32;
        } */ 
       //this is not safe that we check neighbour elements so we will check until st == end == mid 
         if arr[mid+1] > arr[mid]{
            st = mid +1;
        }
        else{
            end = mid;
        }
      }
      st as i32
    }
}