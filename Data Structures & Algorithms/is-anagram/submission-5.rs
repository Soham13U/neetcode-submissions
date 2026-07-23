impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        
        let mut map1 = HashMap::new();
        let mut map2 = HashMap::new();

        for c in s.chars(){
            let count = map1.entry(c).or_insert(0);
            *count += 1;
        }

        for c in t.chars(){
            let count = map2.entry(c).or_insert(0);
            *count += 1;
        }

        map1 == map2
        
    }
}
