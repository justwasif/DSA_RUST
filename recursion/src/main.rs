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
fn main(){
//    let number=recursion_sum(10);
//    println!("{:?}",number);
    let k=fib(10);
    println!("{:?}",k);
}