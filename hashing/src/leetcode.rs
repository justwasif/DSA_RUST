pub fn is_anagram(s: String, t: String) -> bool {
        let mut one=HashMap::new();
        let mut two=HashMap::new();
        for i in s.chars(){
            *one.entry(i).or_insert(0)+=1;
        }
        for j in t.chars(){
            *two.entry(j).or_insert(0)+=1;
        }if one==two{
            return true;
        }
        return false

    }

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

     pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut a=HashMap::new();
        let mut b=HashMap::new();
        let mut c=Vec::new();
        for i in nums1{
            *a.entry(i).or_insert(0)+=1
        }
        for j in nums2{
            *b.entry(j).or_insert(0)+=1
        }
        for i in a.keys(){
            if b.contains_key(i){
                let b_freq=b.get(i).unwrap();
                let a_freq=a.get(i).unwrap();
                if a_freq>b_freq{
                    for l in 0..(*b_freq){
                        c.push(*i);
                    }
                }else if *b_freq>*a_freq{
                    for m in 0..(*a_freq){
                        c.push(*i);
                    }
                }else if b_freq==a_freq{
                    for e in 0..(*a_freq){
                        c.push(*i);
                    }

                }

                

            }
            
        }
        return c;
    }

  pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut to_be_constructed=HashMap::new();
        let mut by_constructed=HashMap::new();
        
        for i in ransom_note.chars(){
            *to_be_constructed.entry(i).or_insert(0)+=1        
        
        }
        for j in magazine.chars(){
            *by_constructed.entry(j).or_insert(0)+=1        
        
        }
        for i in to_be_constructed.keys(){
            if !by_constructed.contains_key(i){
                return false;
            }else  {
                let a=to_be_constructed.get(i).unwrap();
                let b=by_constructed.get(i).unwrap();
                if b<a{
                    println!("{b} ,{a}");
                    return false;
                }
                
            }
            

        }
        return true;
    }
pub fn first_uniq_char(s: String) -> i32 {
    let mut frequence=HashMap::new();
    let mut index=0;
    let mut arr=Vec::new();
    let mut equate_arr=Vec::new();

    for i in s.chars(){
        *frequence.entry(i).or_insert(0)+=1;
        arr.push(i);
    }
    for i in frequence.keys(){
       
        let a=frequence.get(i).unwrap();
        
        if a<&2{
           equate_arr.push(i);
        }
    }
    for i in arr{
        for j in &equate_arr{
            if &&i==j{
                return index;
            }

        }
        index=index+1;
    }
    

    return -1;   
    }


pub fn find_the_difference(s: String, t: String) -> char {
    let mut one_less=HashMap::new();
    let mut one_more=HashMap::new();
    for i in s.chars(){
        *one_less.entry(i).or_insert(0)+=1;

    }
     for i in t.chars(){
        *one_more.entry(i).or_insert(0)+=1;

    }
    let a='k';

    for i in one_more.keys(){
        if !one_less.contains_key(i){
            return *i;
        } 
        if one_less.contains_key(i){
            let less=one_less.get(i).unwrap();
            let more=one_more.get(i).unwrap();
            if more!=less{
                return *i;
            }
            
        }
        
    }
    println!("{:?}, {:?}",one_less,one_more);
    return a;
}

 pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let mut j=HashMap::new();
    let mut s=HashMap::new();
    let mut index=0;
    for i in jewels.chars(){
        *j.entry(i).or_insert(0)+=1;
    }
    for i in stones.chars(){
        *s.entry(i).or_insert(0)+=1;

    }
    for i in j.keys(){
        
        if s.contains_key(i){
            let p=j.get(i).unwrap();
            let o=*s.get(i).unwrap();
        
            for a in 0..o{
            
                index=index+1;
            }
        }
         
    }


     return index;
    }



pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    let arr_len = arr.len();

    let mut freq_count = HashMap::new();
    let mut freq_arr = Vec::new();
    let mut correct_order = Vec::new();

    for i in arr {
        *freq_count.entry(i).or_insert(0) += 1;
    }

    for i in freq_count.values() {
        freq_arr.push(*i);
    }

    for i in 1..=arr_len {
        for j in &freq_arr {
            if i as i32 == *j {
                correct_order.push(i as i32);
            }
        }
    }

    let l = correct_order.len();
    let mut big_bug = 0;

    for i in &correct_order {
        big_bug += 1;
        for j in big_bug..l {
            let equat = &correct_order[j];
            if i == equat {
                return false;
            }
        }
    }

    true
}