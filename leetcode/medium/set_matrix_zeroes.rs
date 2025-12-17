impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {

        if matrix.is_empty() || matrix[0].is_empty() {
           return;
        }

      let row = matrix.len();
      let col = matrix[0].len();

      


      let mut first_row_zero = false;
      let mut first_col_zero = false;

      for i in 0..row{
          if matrix[i][0] == 0{
            first_col_zero = true;
            break;
          }
       }
       for i in 0..col{
        if matrix[0][i] == 0{
            first_row_zero = true;
            break;
        }
       }

       for i in 1..row{
        for j in 1..col{
            if matrix[i][j] == 0{
                matrix[i][0] = 0;
                matrix[0][j] = 0;
             }
        }
       }

      for i in 1..row{
        for j in 1..col{
            if matrix[i][0] == 0 ||   matrix[0][j] == 0{
                matrix[i][j] = 0;
             }
        }
       }

      if first_col_zero == true{
       for i in 0..row{
        
            matrix[i][0] = 0;
        }
       } 

        if first_row_zero == true{
       for i in 0..col{
       
            matrix[0][i] = 0;
        }
       }



    }
}