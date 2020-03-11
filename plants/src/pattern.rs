use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Pattern {
    pub pattern : char,       // Initial character
    pub replacement : String, // Replacement string
    pub p : f32,               // Replacement probability
    pub left : Option<String>,          // Left context
    pub right : Option<String>,         // Right context
}

impl Pattern {
    pub fn new(pat : char, r : String, p : f32, left : Option<String>, right : Option<String>) -> Pattern {
        Pattern{pattern: pat, replacement: r, p, left, right }
    }

    pub fn test(&self, i : usize, s : String) -> bool {

        let mut rng = thread_rng();
        if rng.gen_bool(self.p.into()) {
            //if (self.left == ' ') && (self.right == ' ') {  // No context
                s.chars().nth(i).unwrap() == self.pattern
            /*}
            else if (self.left != ' ') && (self.right != ' ') {  // Both contexts
                if i <= 0 || i >= s.len() - 1 {
                    false
                }
                else {
                    //println!("s : {}, s.len() : {}, index : {}", s, s.len(), i);
                    s.chars().nth(i-1).unwrap() == self.left
                        && s.chars().nth(i).unwrap() == self.pattern
                        && s.chars().nth(i+1).unwrap() == self.right
                }
            }
            else if self.right != ' ' {  // Right context only
                if i == s.len() - 1 {
                    false
                }
                else {
                    s.chars().nth(i).unwrap() == self.pattern
                        && s.chars().nth(i+1).unwrap() == self.right
                }
            }
            else if self.left != ' ' {  // Left context only
                if i == 0 {
                    false
                }
                else {
                    s.chars().nth(i).unwrap() == self.pattern
                        && s.chars().nth(i-1).unwrap() == self.left
                }
            }
            else {
                false
            }*/
        }
        else {
            false
        }
    }
}
