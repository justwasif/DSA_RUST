#![allow(dead_code)]      // item exists but is never used
#![allow(unused_variables)] // local variables not used
#![allow(unused_imports)] // imports not used
#![warn(unconditional_recursion)]
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

  

fn main() {
    // let mut k=["a","b","c","d","l","k"];
    // let mut f=for_brute(&k);
    // let mut g=hash_met(&k);
    // println!("{:?}",f);
    // println!("{:?}",g);
    
    // println!("{:?}",count("let let it it be the no one nikhar".to_string()));

    
    
}
