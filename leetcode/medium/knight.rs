impl Solution {
    pub fn check_valid_grid(grid: Vec<Vec<i32>>) -> bool {
         if grid[0][0] != 0 {
            return false;
        }
        ispossible(&grid ,0 , 0,  0)
    }
}

 fn ispossible(grid : & Vec<Vec<i32>> , row : i32 , col : i32 ,  expval: i32)-> bool{
    let n = grid.len() as i32;
    if row < 0 || col < 0 || row >= n || col >= n || grid[row as usize][col as usize] != expval{
        return false;
    } 
    if expval == (n*n) -1  {
        return true;
    }

    let ans1 = ispossible(grid , row - 2 , col + 1 , expval + 1);
    let ans2 = ispossible(grid , row - 1 , col + 2 , expval + 1);
    let ans3 = ispossible(grid , row + 1 , col + 2 , expval + 1);
    let ans4 = ispossible(grid , row + 2 , col + 1 , expval + 1);
    let ans5 = ispossible(grid , row + 2 , col - 1 , expval + 1);
    let ans6 = ispossible(grid , row + 1 , col - 2 , expval + 1);
    let ans7 = ispossible(grid , row - 1 , col - 2 , expval + 1);
    let ans8 = ispossible(grid , row - 2 , col - 1 , expval + 1);

    return (ans1 || ans2 || ans3 || ans4 || ans5 || ans6 || ans7 || ans8 );
 }