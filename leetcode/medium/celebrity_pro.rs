fn find_celebrity_matrix(matrix: Vec<Vec<i32>>) -> i32 {
    let n = matrix.len();
    if n == 0 { return -1; }

    let mut stack: Vec<usize> = (0..n).collect();

    
    while stack.len() > 1 {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();

        
        if matrix[a][b] == 1 {
            stack.push(b);
        } else {
            
            stack.push(a);
        }
    }

 
    let candidate = stack.pop().unwrap();

    for i in 0..n {
        if i == candidate { continue; }

        if matrix[candidate][i] == 1 || matrix[i][candidate] == 0 {
            return -1;
        }
    }

    candidate as i32
}