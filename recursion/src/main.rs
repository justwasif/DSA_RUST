#![allow(dead_code)]      // item exists but is never used
#![allow(unused_variables)] // local variables not used
#![allow(unused_imports)] // imports not used

use std::array;
use std::fmt::Alignment::Right;
use std::fs;
use std::iter::Product;
use std::path;

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

fn parameterised_sum(number:i32,sum:i32){
    if number<1{
        println!("{}",sum);
        return;
    }
    parameterised_sum(number-1, sum+1);
}

fn parameterised_factroial(i:i32,fact:i32){
  
    if i<1{
         println!("{fact}");
         return;
    }
        parameterised_factroial(i-1,fact*i);
   
}
fn parameterised_array(arr: &[i32]){
    let mut  Product=1 ;
    for i in arr{
        Product=Product*i;
        
    }
    println!("{}",Product);
   
}

fn entre_exit(i:i32){
    if i>10{
       
        return;
    }
    println!("entre {i}");
    entre_exit(i+1);
     println!("exit {i}");


}

fn power(i:i32,k:i32)->i32{
    if k<1{
        return 1;
    }

    return i*power(i, k-1);
    
}
fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    None
}


fn main(){
//    let number=recursion_sum(10);
//    println!("{:?}",number);
    // let k=fib(10);
    // println!("{:?}",k);
    // let repetation =print_name(10, 1) ;
    // print!("{:?}",repetation);
    // let a=parameterised_sum(10, 0);
    // println!("{:?}",a);
    // let a=parameterised_factroial(5,1);
    // println!("{:?}",a);
    // let k=[1,2,3,4,5,6];
    // let a=parameterised_array(&k);
    // println!("{:?}",a);
    // let a = entre_exit(1);
    // println!("{:?}",a);
    // let a=power(2, 4);
    // print!("{:?}",a);
    let ar=[0,1,2,3,4,5,6,7,8,9];
    let a=binary_search(&ar, 8);
    println!("{:?}",a);

}