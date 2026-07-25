impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut s2: String = s.chars().rev().flat_map(|c| c.to_lowercase()).collect();
        let cleaned_s: String = s.chars().filter(|c| c.is_alphanumeric()).flat_map(|c| c.to_lowercase()).collect(); 
     
        s2.retain(|c| c.is_alphanumeric());



        cleaned_s == s2
    }
}
