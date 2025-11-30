use std::io;


fn main() {
    println!("Hello, world!");

    println!("Enter a number for FizzBuzz:");
    let mut no = String::new();
   io::stdin().read_line(&mut no).unwrap();
   let no = no.trim().parse::<u32>().unwrap();


    fizzbuzz(no);
    factorial(no);
    fibonacci(no);
   
}

fn fizzbuzz(n: u32) {
    for i in 1..n {
        if i%3 == 0{
            println!("Fizz");

         }
         else if i%5 == 0  {
             println!("Buzz")
         }
         else if i%15 == 0 /*i%3==0 && i%5 == 0 */ {
             println!("FizzBuzz")
         }
         else {
             println!("{i}")
         }
    }
}

fn factorial(n:u32) {
  let mut result = 1;
   for i in 1..=n{
    result *= i;
   }
   println!("{result}") // remember return values from functions so they can be used further 
}


fn fibonacci(n: u32){

    if n == 0{
        return
    }
    else if n == 1{
        println!("0");
    }
    let mut num: u32 = 0 ;
    
    let mut num1 :u32= 1;
    println!("num\nnum1");
    let mut result : u32;
   for _ in 1..n-1{ //you don't need to give a variable for just iteration give it when you need index otherwise just give "_"
    result = num+num1;
    num = num1;
    num1 = result;
    println!("{result}");

   }
}