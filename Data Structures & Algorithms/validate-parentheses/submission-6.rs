impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars(){
            if c == '[' || c == '{' || c == '('{
                stack.push(c);
            }
            else{
                match stack.last() {
                    Some(&'[') if c == ']' => { stack.pop(); }
                    Some(&'(') if c == ')' => { stack.pop(); }
                    Some(&'{') if c == '}' => { stack.pop(); }
                    _ => return false, 
                }
            }

            }
              stack.is_empty()
        }
      
    }

/*
vec 
scan element
if opening bracket
    push element
else
    if bracket  match 
        pop
    else
        return false


return is vec empty


*/