use std::io;
fn main() {
   
    let mut b: [i32; 2] = [0, 0];
    b[0] = 23;
    println!("{:?}", b);

    let  a = [12, 34, 6 , 4 ];
    println!("{:?}",a);
    
    let arr = _2d_arr_input();
    println!("{:?}",arr);

    let largest = largestno_2d_arr(arr);
    println!("{largest}");

    reverse_arr(a);
    
}

fn _2d_arr_input() -> Vec<Vec<i32>>{
    // multi dimesional

    println!("Enter value in 2d array");
    let mut arr = vec![vec![0; 3]; 3];

    for x in 0..arr.len() {
        // here x will take refrence to  first row as linear array
        for y in 0..arr[0].len() {
            // then y will be  refrence to each element of that linear array
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            arr[x][y] = match input.trim().parse::<i32>(){
               Ok(input) => input,
               Err(_) =>{
                  println!("Don't press enter without giving input and expected integer");
                  continue
               }
            };

            
        }
    }
    return arr
}

fn largestno_2d_arr(arr: Vec<Vec<i32>> ) -> i32{
    let mut result = i32::MIN; //it should me assign as i32::MIN as it is possible to put a negative integers
       //largest in array 
        for x in arr {
        // here x will take refrence to first row as linear array
        for y in x {
           if result < y {
            result = y;
           }
        }
    }
      result
       
}
fn reverse_arr(mut a: [i32;4]) -> [i32;4] {
    
        let last = a.len()-1;
   for i in 0..a.len()/2{
      /* bucket = a[i];
      a[i] = a[last];
      a[last] = bucket;*/ 
      
       a.swap(i,last-i); // swap function used for swapping 
     }

   return a

}