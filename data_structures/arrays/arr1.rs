use std::io;
fn main() {
    let mut _a = [12, 34, 6 , 4 ];
  //  let mut bucket = 0;
    let mut  last = _a.len()-1;
   for i in 0.._a.len()/2{
       if i > last/2 {break}; 
     /* bucket = _a[i];
      _a[i] = _a[last];
      _a[last] = bucket;*/

      _a.swap(i,last-i);
     
      
   }
   println!("{:?}",_a);
    let mut b: [i32; 2] = [0, 0];
    b[0] = 23;
    println!("{:?}", b);

    let mut v = Vec::new();
    v.push(2);
    println!("{:?}", v);

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

        
       println!("{:?}",arr);
   let mut result = i32::MIN; //it should me assign as i32::MIN as it is possible to put a negative integers
       //largest in array 
        for x in arr {
        // here x will take refrence to  first row as linear array
        for y in x {
           if result < y {
            result = y;
           }
        }
    }
    println!("Largest no. {result}");
       
    let mut ve: Vec<Vec<i32>> = Vec::new();
    ve.push(vec![1, 2, 3, 4, 5]);
    ve.push(vec![1, 2, 3, 5, 6]);

    
    for (i, v) in ve.iter_mut().enumerate() {
        for (x, y) in v.iter_mut().enumerate() {
        // if you are using iter() you can't mutate value
        //use iter_mut to mutate elements
         *y  *= 2; // needs deref as we are geting &mut i32
            println!("Index [{}][{}] , Value : {}", i, x, y)
        }
    }
    for i in 0..ve.len() {
        for y in 0..ve[0].len() {
           
            println!("Index [{}][{}]", i, y);
        }
    }
    print!("{:?}", ve);
}
