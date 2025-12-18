impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
       if intervals.is_empty(){
          return vec![];
       }

       let mut merge : Vec<Vec<i32>> = Vec::new();


       // sorting of all rows by first element of each row
       // |i| clouser like one line function with argument i and i is row with i[0] first element of that row 
       intervals.sort_by_key(|i| i[0]);

       for interval in intervals{

        //check for merge sort empty or last row inserted in merge has end  element less than first element of current interval or row 
        // if true just push current interval no overlapping
        //merge.last().unwrap()[1] for last element or row insterted and [1] is 2nd or eend of that row
        if merge.is_empty() || interval[0] > merge.last().unwrap()[1]{
            merge.push(interval);
        }


        else {
            // taking last merge elem as mutable 
            let last = merge.last_mut().unwrap();
            // end or 2nd element of row that we stored last change with max betweenn current end and last end for overlapping
            last[1] = last[1].max(interval[1]);
        }
       }
     merge

    }
}

//time complexity is O(nlogn + n) => O(nlogn)

