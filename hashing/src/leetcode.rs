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
