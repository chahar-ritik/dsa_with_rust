fn main() {


    let mut v = Vec::new();
    v.push(2);
    println!("{:?}", v);

    let mut vec = vec!['a','b','c'];
    vec.push('d');// each time when we push something in vector firtly its capacity increase x2 times then element stored 
    vec.push('h');
    // here capacity increases first time when pushed to 6 
    // So when we pushed second time we have space no need to extend capacity
    println!("Capacity : {}",vec.capacity());
    println!("Element at index [3] {}",vec[3]);
    
    // remove value at given index. it is not like pop that remove value at top
    vec.remove(2); 
    //insert value to given index but index should be in bound or at top
    vec.insert(1, 'e');
    println!("{:?}", vec);

    let x = vec![3;5]; // when we are giving 3 as element 5 time
    println!("{:?}", x);

    //multi dimensional vector or we can say growable array
    let mut ve: Vec<Vec<i32>> = Vec::new();
    ve.push(vec![1, 2, 3, 4, 5]);
    ve.push(vec![1, 2, 3, 5, 6]);
      

     // we can also use index for applying any algorithm as in other languages like c++
    
    for i in 0..ve.len() {
        for y in 0..ve[0].len() {
            println!("Index [{}][{}]", i, y);
        }
    }
    println!("{:?}", ve);
    
    // mutate each element with iter_mut()
    for (i, v) in ve.iter_mut().enumerate() {
        for (x, y) in v.iter_mut().enumerate() {
            // if you are using iter() you can't mutate value
            //use iter_mut to mutate elements
            *y *= 2; // needs deref as we are geting &mut i32
            println!("Index [{}][{}] , Value : {}", i, x, y)
        }
    }

   
}
