impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
         if matrix.is_empty() { return vec![]; }
        let mut srow = 0;
        let mut erow = matrix.len()-1;
        let mut scol = 0;
        let mut ecol = matrix[0].len()-1;
        let mut mat = Vec::new();
        
        while srow <= erow && scol <= ecol {
        //top
        for i in scol..=ecol {
             mat.push(matrix[srow][i]);
        }
        //right
        for i in srow+1..=erow {
             mat.push(matrix[i][ecol]);
        }
        //bottom
         if srow < erow{
        for i in (scol..ecol).rev() {
             mat.push(matrix[erow][i]);
        }
     } 
        //left
         if scol < ecol{
        for i in (srow+1..erow).rev() {
             mat.push(matrix[i][scol]);
        }
     }
         srow +=1;
         erow = erow.saturating_sub(1);
         scol +=1;
         ecol = ecol.saturating_sub(1);
        }

        mat
    }
}