fn quick_sort(nums: &mut Vec<i32> , st: usize, end: usize){
    if st < end{
        let pivot_idx = partation(nums , st , end);
        //left
        if pivot_idx > 0 {
        quick_sort(nums , st , pivot_idx-1 );
        }
        //right
        quick_sort(nums ,  pivot_idx+1 , end);
    }
}

fn partation(nums: &mut Vec<i32> , st: usize , end: usize)-> usize{
    let pivot = nums[end];
    let mut idx = st;
    for j in st..end{
        if nums[j] <= pivot {
          nums.swap(idx , j);
          idx+=1;
        }
    }

    nums.swap(idx , end);
    idx

}

fn main(){
    let mut nums = vec![12,32,35,8,31,17];

    if nums.len()>1{
        let n = nums.len()-1;
        quick_sort(&mut nums , 0 , n);
           println!("{:?}",nums);

    }else{
        println!("{:?}",nums);
    }
}