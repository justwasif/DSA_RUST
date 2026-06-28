#![allow(dead_code)]      // item exists but is never used
#![allow(unused_variables)] // local variables not used
#![allow(unused_imports)] // imports not used
#![warn(unconditional_recursion)]
#![warn(non_snake_case)]
use std::collections::HashMap;

fn hash(){
    let mut f=HashMap::new();
    f.insert(1, 2);
}

fn count(sentence:String)->HashMap<String,i32>{
    let mut s=HashMap::new();
    for word in sentence.split_whitespace(){
        let word=word.to_string();
        *s.entry(word).or_insert(0)+=1;
    }
    
    return s;


} 
fn for_brute(arr:&[&str])->bool{
   for i in arr{

    if i==&"l"{
        return true;
        }
    }
   return  false;
}
fn hash_met(arr:&[&str])->bool{
    let mut s=HashMap::new();
    for i in arr{
        *s.entry(i).or_insert(0)+=1

    }
    if s.contains_key(&"l"){
        return true;
    }
    return false;

 }

 fn arr_v(){
    let num=vec![1,2,3,4,5,3,4,5];
    let mut hash=vec![0;6];
    for nums in num {
        hash[nums as usize]+=1;
    }
    println!("{:?}",hash);

 }
  

fn arr_map(arr:&[i32])->Vec<i32>{
    let mut hash=vec![0;9];
    for &i in arr{
        hash[i as usize]+=1;
    }
    return hash;

}

// fn freq(num:&[usize],max_val:i32)->Vec<usize>{
//     let mut freq=vec![0;max_val+1];
//     for &i in num {
//         freq[i]+=1;
//     }
//     return freq;
// }
fn freq_arr(num:&[usize])->Vec<i32>{
    let mut freq=vec![0;50];
    for &i in num{
        freq[i]+=1;
    }
    return freq;
}

fn q_n_q()->bool{
    let mut k=HashMap::new();
    for i in 0..1000001{
        *k.entry(i).or_insert(0)+=1;
    }
    if k.contains_key(&1000000){
        return true;
    }else {
        return false;
    }
}

fn q_nq()->bool{
    let mut arr=Vec::new();
    let mut l=0;
    for i in 1..1000001{
        arr.push(i);
    }

    while l<arr.len(){
        if arr[l]==1000000{
            return true;
        }
        l=l+1
    }
    return false;

}

fn char_count(){
    let mut freq=[0;26];
    let s="abcaddjdj";
    for ch in s.chars(){
        let x=(ch as u8-b'a') as usize;
        freq[x]+=1;
    }
    println!("{:?}",freq[0]);
}

fn spec_count(word:&str){
    let mut freq=[0;26];
    for ch in word.chars(){
        let x=(ch as u8-b'a') as usize;
        freq[x]+=1;
    }
    for (i,count) in freq.iter().enumerate(){
        let ch=(b'a'+i as u8) as char;
        println!("{},{}",ch,count);
    
    }
}

fn main() {
    // let arr=vec![0i32,10000000];
    // println!("{:?}",(arr.len() * std::mem::size_of::<i32>()) as usize
    //         as f64 / (1024.0 * 1024.0));
    println!("{:?}",spec_count("mississippi"));
   
    
}
