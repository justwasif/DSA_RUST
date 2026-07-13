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
fn insertion_sort(arr:&mut[i32]){
    let n=arr.len();
    for i in 1..n{
        let key=arr[i];
        let mut j=i;
        while j>0&& arr[j-1]>key{
            arr[j]=arr[j-1];
            j-=1;
        }
        arr[j]=key;
    }
}

fn insertion_sort_string(arr:&mut[String]){
    let n=arr.len();
    for i in 1..n{
        let key=arr[i].clone();
        let mut j=i;
        while j>0&&arr[j-1]>key{
            arr[j]=arr[j-1].clone();
            j=j-1;


        }
        arr[j]=key;

    }

}
fn integer_shifiting(arr:&mut[i32]){
    let n=arr.len();
    for i in 1..n{
        let key=arr[i];
        let mut j=i;
        while j>0 &&arr[j-1]>key{
            arr[j]=arr[j-1];
            j=j-1;

        }
        arr[j]=key;
    }
}
fn main() {
    let arr=&mut[4,5,1,9,8,0,3];
    integer_shifiting(arr);
    println!("{:?}",arr);

}
