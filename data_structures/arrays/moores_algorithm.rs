// majority elements that appear more then n/2 time in an array

// if only one solution given 

fn main(){
    let a = [0,0,1,2,2,1,2,3,4,1,1];
    println!("{}",maj_ele(&a));
}

fn maj_ele(a : &[i32]) -> i32 {
   let mut  freq = 0; 
   let mut ans = 0;

   for i in 0..a.len(){
     if freq == 0 {
        ans = a[i]
     }
     else if ans == a[i] { freq+=1;}
     else {freq-=1;}
   }
   return ans

}