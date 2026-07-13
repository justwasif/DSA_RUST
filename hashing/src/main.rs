#![allow(dead_code)]      // item exists but is never used
#![allow(unused_variables)] // local variables not used
#![allow(unused_imports)] // imports not used
#![warn(unconditional_recursion)]
#![warn(non_snake_case)]
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::time::Instant;


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

fn counting_programming(word:&str)->Vec<i32>{
    let mut freq=vec![0;26];
    for i in word.chars(){
        let x=(i as u8-b'a') as usize;
        freq[x]+=1;
    }
    freq

}

fn mapping(){
    let nums=vec![1,999999999,42];
    let mut freq=HashMap::new();
    for i in nums{
        *freq.entry(i).or_insert(0)+=1;
    }
    println!("{:?}",freq);

}

fn btreeimplementation(){
    let mut map=BTreeMap::new();
    map.insert(5, 100);
    map.insert(1, 50);
    println!("{:?}",map);
}
// btree automaticall does sort it through key order
fn btreecount(arr:&[i32]){
    let mut map=BTreeMap::new();
    let mut start=0;
    for i in arr{
        map.insert(start, i);
        start=start+1;

    }
    println!("{:?}",map);

}

fn count_23(arr:&[i32]){
    let mut h=HashMap::new();
    for i in arr{
        *h.entry(i).or_insert(0)+=1;
    }

}





fn hash_j(){
    let mut h=HashMap::new();
    h.insert("a",22);
} 

fn capital_country(arr:&[&str],barr:&[&str]){
    let mut a=HashMap::new();
    for (i,j) in arr.iter().zip(barr.iter()){
        a.insert(i, j);
    }
    println!("{:?}",a);
    
}

fn comparing_btree(){
    let start=Instant::now();
    let mut btree=BTreeMap::new();
    for i in 0..1000000{
        btree.insert(i, i);
    }
    println!("{:?}",start.elapsed());

}
fn comparing_hash(){
    let srart=Instant::now();
    let mut hash=HashMap::new();
    for i in 1..1000000{
        hash.insert(i, i);
    }
    println!("{:?}",srart.elapsed())

}
fn toy_hash(){
    let arr=[12,22,32,43,52];
    let mut hash=HashMap::new();
    for &i in &arr{
        let k= (i % 10) as usize;
        *hash.entry(k).or_insert(0)+=1;
    }

}
fn toy_chain_hash_table(arr:&[i32]){
    let mut table=vec![vec![];10];
    for i in arr{
        table[8].push(i);
    }

}
// pub fn contains_duplicate(nums: Vec<i32>) -> bool {
//         fn hashmethod(num:Vec<i32>)->bool{
//             let mut h=HashMap::new();
//             for i in num{
//                 let mut freq=h.entry(i).or_insert(0);
//                 *freq+=1;
//                 if *freq>1{
//                     return true;
//                 }

//             }
//             return false;
            
//         }
//         hashmethod(nums)
//     }

   pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut a=HashMap::new();
    let mut b=HashMap::new();
    let mut c=Vec::new();
    for i in nums1{
        a.entry(i).or_insert(1);
    }
    for j in nums2{
        b.entry(j).or_insert(1);
    }
    for k in a.keys(){
        if b.contains_key(k){
            c.push(*k);
        }
        
    }
    return c;
        
    }

//phele i lete hai vec i.chars() hashmap 
    
 pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let 
        
    }
fn main() {
   
   
   
    
   
   
}
