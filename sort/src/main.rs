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
// fn merge_sort(arr:&mut[i32]){
    let n=arr.len();
    if n<=1{
        return;

    }
    let mid=n/2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);
    let mut i=0;
    let mut j =0;
    let (left,right)=arr.split_at(mid);

    let mut merged=Vec::with_capacity(n);

    while i<left.len() && j<right.len(){
        if left[i]<=right[j]{
            merged.push(left[i]);
            i+=1;
            
        }else {
            merged.push(right[j]);
            j+=1;
        }

    }
    while i<left.len(){
        merged.push(left[i]);
        i+=1;
    }
    while j<right.len(){
        merged.push(right[j]);
    }
    arr.copy_from_slice(&merged);
    
}

fn merge(arr: &mut [i32], low: usize, mid: usize, high: usize) {
    let mut temp = Vec::with_capacity(high - low + 1);

    let mut left = low;
    let mut right = mid + 1;

    while left <= mid && right <= high {
        if arr[left] <= arr[right] {
            temp.push(arr[left]);
            left += 1;
        } else {
            temp.push(arr[right]);
            right += 1;
        }
    }

    while left <= mid {
        temp.push(arr[left]);
        left += 1;
    }

    while right <= high {
        temp.push(arr[right]);
        right += 1;
    }

    for i in low..=high {
        arr[i] = temp[i - low];
    }
}

fn merge_sort(arr: &mut [i32], low: usize, high: usize) {
    if low >= high {
        return;
    }

    let mid = low + (high - low) / 2;

    merge_sort(arr, low, mid);
    merge_sort(arr, mid + 1, high);

    merge(arr, low, mid, high);
}

fn main() {
    let mut arr = vec![3, 1, 2, 4, 1, 5, 2, 6, 4];

    let n = arr.len();

    merge_sort(&mut arr, 0, n - 1);

    println!("{:?}", arr);
}


