#![allow(dead_code)]      // item exists but is never used
#![allow(unused_variables)] // local variables not used
#![allow(unused_imports)] // imports not used
fn multiply(a:i32,b:i32)->i32{
    a*b
}

fn recursion(n:i32){
    if n==0{
        return;
    }
    println!("{}",n);
    recursion(n-1);
}
fn recursion_sum(n:i32)->i32{
    if n==0{
        return 0;
    }
    n + recursion_sum(n-1)

    
}
fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }

    fib(n - 1) + fib(n - 2)
}

fn print_name(count:i32,counter:i32){
    if counter>count{
        return;
    }
    println!("name");
    print_name(count, counter+1);

}
fn main(){
//    let number=recursion_sum(10);
//    println!("{:?}",number);
    // let k=fib(10);
    // println!("{:?}",k);
    let repetation =print_name(10, 1) ;
    print!("{:?}",repetation);
}