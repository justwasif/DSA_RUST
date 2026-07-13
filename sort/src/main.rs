#![allow(dead_code)]      // item exists but is never used
#![allow(unused_variables)] // local variables not used
#![allow(unused_imports)] // imports not used
#![warn(unconditional_recursion)]
#![warn(non_snake_case)]
use std::cmp::Ordering;
use std::collections::HashMap;

fn sort(arr:Vec<i32>)->bool{
    if arr.is_sorted()==true{
        return arr.is_sorted();
    }
    if arr.is_sorted_by(|a,b| a>=b)==true{
        return arr.is_sorted_by(|a,b| a>=b);
    }
    return false;
    
}

fn selection_sore(arr:&mut[i32]){
    let n=arr.len();
    for i in 0..n{
        let mut min=i;
        for j in (i+1)..n{
            if arr[j]<arr[min]{
                min=j;
            }
        }
        arr.swap(i,min);
    }
    
}

fn selection_sort_no_swap(arr:&mut[i32]){
    let n=arr.len();
    for i in 0..n{
        for j in (i+1)..n{
            if arr[j]<arr[i]{
                let new=arr[i];
                arr[i]=arr[j];
                arr[j]=new;
            }
        }
        
    }
    println!("{:?}",arr);
}
fn main() {
    let mut arr=&mut[3,5,6,7,9,1,2,0];
    println!("{:?}",selection_sort_no_swap(arr));

}
