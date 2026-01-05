//n_queens problem is for n queens on a n*n size board and Place the queen that they can't attack each other with move in original chess board

fn is_safe(board: & Vec<String>, row: usize , col: usize , n : usize) -> bool{
   // check for column but no need to check for row as one queen each row and then row updated
    for r in 0..row{
       if board[r].as_bytes()[col] == b'Q'{
           return false;
       }

    }

    //check for upper left diagonal
     let mut r1 = row as i32 - 1;
     let mut c1 = col as i32 -1;

    while r1 >= 0 && c1 >= 0{
        if board[r1 as usize].as_bytes()[c1 as usize] == b'Q'{
            return false;

        }
        r1 -=1;
        c1 -=1;

    }

    //upper right diagonal
     let mut r2 = row as i32 - 1;
     let mut c2 = col as i32 + 1;

    while r2 >= 0 && c2 < n as i32{
        if board[r2 as usize].as_bytes()[c2 as usize] == b'Q'{
            return false;

        }
        r2 -=1;
        c2 +=1;

    }

    true

 
}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
      
       fn n_queens(board: &mut Vec<String>, ans: &mut Vec<Vec<String>>, row : usize, n: usize ){
        if row == n {
            ans.push(board.clone());
            return;
        }
          for col in 0..n {
            if is_safe(board , row , col , n ){
            let mut chars : Vec<char> = board[row].chars().collect();
             chars[col] = 'Q';
             board[row] = chars.iter().collect();
             n_queens(board , ans , row+1 , n);
             // backtracking
             chars[col] = '.';
             board[row] = chars.iter().collect();

            }

          }

       }

        let n = n as usize;
        let dots : String = ".".repeat(n);
        let mut board :Vec<String> = vec![dots.clone();n];
        let mut ans :Vec<Vec<String>> = Vec::new();
        
        n_queens(&mut board, &mut ans, 0, n );

        ans
    }
}