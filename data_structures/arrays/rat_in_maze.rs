fn rat_in_maze(maze: &mut Vec<Vec<i32>> ) -> Vec<String> {
    let mut ans : Vec<String> = Vec::new();
    let mut path  = String::new();
    
    fn search_path(maze : &mut Vec<Vec<i32>>, r: i32 , c:i32 , ans: &mut Vec<String> , path : &mut String){
     let n = maze.len();
     if r == (n-1) as i32 &&  c == (n-1) as i32{
        ans.push(path.clone());
        return;
     }
     if r < 0 || c < 0 || r >= n as i32 || c >= n as i32 || maze[r as usize][c as usize] == 2 || maze[r as usize][c as usize] == 0{
        return;
     }

     let temp = maze[r as usize][c as usize];
      maze[r as usize][c as usize] = 2;

     //Up
     path.push('U');
     search_path(maze , r-1 , c , ans , path);
     path.pop();
     //Down
     path.push('D');
     search_path(maze , r+1 , c , ans , path);
     path.pop();
     
     //Left
     path.push('L');
     search_path(maze , r , c-1 , ans , path);
     path.pop();
     //Right
     path.push('R');
     search_path(maze , r , c+1 , ans , path);
     path.pop();

    maze[r as usize][c as usize] = temp;

    }

    search_path(maze , 0  , 0 , &mut ans , &mut path);

    ans
}

fn main(){
    let mut maze : Vec<Vec<i32>> =  vec![
        vec![1, 0, 0, 0], 
        vec![1, 1, 0, 1], 
        vec![1, 1, 0, 0], 
        vec![0, 1, 1, 1]];
    
    let expected_output = [String::from("DDRDRR"), String::from("DRDDRR")];
     assert_eq!(rat_in_maze(&mut maze),expected_output);
    println!("{:?}" , rat_in_maze(&mut maze));
   

}