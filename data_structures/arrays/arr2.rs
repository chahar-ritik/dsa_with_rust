fn main(){
  let arr = [2,4,5,38];
  
  let s = sum_and_prod(arr);
  println!("Sum:{},Product:{} ",s.0,s.1);

  let swapped_arr = swap_minmax(arr);
  println!("{:?}",swapped_arr);




}

fn sum_and_prod(a: [i32;4]) -> (i32, i32){

     let mut sum = 0;
     let mut prod = 1;
   for i in &a {
     sum += i;
     prod *= i;
   }
  return (sum , prod)
}

fn swap_minmax(mut arr: [i32;4]) -> [i32;4]{
    let  mut m1 = i32::MAX;
    let mut max_i : usize = 0;
    let mut m2 = i32::MIN;
    let mut min_i : usize = 0;
     
    for i in 0..arr.len(){
        if m1 > arr[i]{
            m1 = arr[i];
          min_i = i;
        }

        if m2 < arr[i] {
              m2 = arr[i];
          max_i = i;
        }
    }

    arr.swap(min_i , max_i);

    arr
}

