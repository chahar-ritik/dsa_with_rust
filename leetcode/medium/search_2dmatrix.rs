impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {

        let row = matrix.len() as i32;
        let col = matrix[0].len() as i32;
     let mut left = 0;
     let mut right = row*col-1;

     while left < right{
        let mid = (left +(right-left)/2);
        let value = matrix[(mid/col )as usize][(mid%col) as usize];
        if value == target {
            return true
        }
        else if value < target{
            left = mid+1;

        }
        else {
            right = mid ;
        }
     }
     return false
    }
}