impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {

        let stack : Vec<usize> = Vec::new();
        let max_area = 0;
        let n = heights.len();


        for i in 0..=n{
            let curr_height = if i==n {
                0 // that will flush the stack
            }else{
                heights[i];
            };
            while let Some(&top) = stack.last(){
                if curr_height < heights[top]{ // this sets the right boundary
                    stack.pop();

                    let height = heights[top];
                    let width = if let Some(&prev) = stack.last(){
                        i-prev-1 // -1 because of right boundary index should be excluded
                    }else{
                        i
                    };

                    max_area = max_area.max(height*width as i32);
                }else{
                    break; // inner while loop break
                }
            }
            stack.push(i); 
        }
        return max_area;
    }
}