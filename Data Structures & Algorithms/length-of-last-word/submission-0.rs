impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut c:i32 = 0;
        for b in s.chars().rev(){
            if b == ' ' && c == 0{
                continue;
            }
            else if b == ' '{
                break;
            }
            else{
                c += 1;
            }
        }
        c
    }
}
/*
iterator , rev over s

*/