use std::collections::{HashMap,HashSet};


fn main(){
  let arr = [2,4,2,38];
  let arr_1 = [ 3,4,38];
  
  let s = sum_and_prod(arr);
  println!("Sum:{},Product:{} ",s.0,s.1);

  let swapped_arr = swap_minmax(arr);
  println!("{:?}",swapped_arr);


  let u = print_unique(arr);
  println!("{:?}",u);

  let vec = intersection_2arr(&arr,&arr_1);
  println!("{:?}",vec);

  let vec_1 = intersection(&arr,&arr_1);
  println!("{:?}",vec_1);

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


fn print_unique(arr: [i32;4]) -> Vec<i32>{
   let mut h = HashMap::new();
   for i in arr{
     *h.entry(i).or_insert(0)+=1;
   }

    h.keys().cloned().collect()
}


// this works but used nested loop that is inefficient
//We can use Hashset
fn intersection_2arr(a: &[i32] , b: &[i32]) -> Vec<i32>{
    let mut v : Vec<i32> = Vec::new();
    for &i in a {
      for &j in b {
        if i == j {
          v.push(i);
        }
      }
    }
    v
}


//with hashset time complexity is less 
fn intersection(a: &[i32] , b: &[i32]) -> Vec<i32>{
  let set_a : HashSet<i32> = a.iter().cloned().collect();
  let set_b : HashSet<i32> = b.iter().cloned().collect();


  set_a.intersection(&set_b).cloned().collect()
}


