use rand::{thread_rng, Rng};
use std::str::Chars;
use std::iter::Rev;

#[derive(Debug)]
pub struct Pattern {
    pub pattern : char,       // Initial character
    pub replacement : String, // Replacement string
    pub p : f32,               // Replacement probability
    pub left : Option<String>,          // Left context
    pub right : Option<String>,         // Right context
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::BorrowMut;

    #[test]
    fn rctx_true() {
        let s = String::from("bc");
        let ctx = String::from("bc");
        let res = Pattern::rctx(s.chars(), ctx.chars().borrow_mut());

        assert!(res);
    }

    #[test]
    fn rctx_false() {
        let s = String::from("bc");
        let ctx = String::from("d");
        let res = Pattern::rctx(s.chars(), ctx.chars().borrow_mut());

        assert!(!res);
    }

    #[test]
    fn rctx_true_short() {
        let s = String::from("bc");
        let ctx = String::from("b");
        let res = Pattern::rctx(s.chars(), ctx.chars().borrow_mut());

        assert!(res);
    }

    #[test]
    fn rctx_false_start() {
        let s = String::from("aabc");
        let ctx = String::from("bc");
        let res = Pattern::rctx(s.chars(), ctx.chars().borrow_mut());

        assert!(!res);
    }

    #[test]
    fn lctx_true() {
        let s = String::from("bc");
        let ctx = String::from("bc");
        let res = Pattern::lctx(s.chars().rev(), ctx.chars().rev().borrow_mut());

        assert!(res);
    }

    #[test]
    fn lctx_false() {
        let s = String::from("bc");
        let ctx = String::from("d");
        let res = Pattern::lctx(s.chars().rev(), ctx.chars().rev().borrow_mut());

        assert!(!res);
    }

    #[test]
    fn lctx_true_short() {
        let s = String::from("abc");
        let ctx = String::from("bc");
        let res = Pattern::lctx(s.chars().rev(), ctx.chars().rev().borrow_mut());

        assert!(res);
    }

    #[test]
    fn lctx_false_start() {
        let s = String::from("bca");
        let ctx = String::from("c");
        let res = Pattern::lctx(s.chars().rev(), ctx.chars().rev().borrow_mut());

        assert!(!res);
    }
}

impl Pattern {
    pub fn new(pat : char, r : String, p : f32, left : Option<String>, right : Option<String>) -> Pattern {
        Pattern{pattern: pat, replacement: r, p, left, right }
    }

    fn rctx(it : Chars, ctx : &mut Chars) -> bool {
        let mut cur = match ctx.next() {
            Some(c) => c,
            None => return true
        };

        for c in it {
            if c == cur {
                cur = match ctx.next() {
                    Some(c) => c,
                    None => return true
                };
            }
            else {
                return false;
            }
        }

        false
    }

    fn lctx(it : Rev<Chars>, ctx : &mut Rev<Chars>) -> bool {
        let mut cur = match ctx.next() {
            Some(c) => c,
            None => return true
        };

        for c in it {
            if c == cur {
                cur = match ctx.next() {
                    Some(c) => c,
                    None => return true
                };
            }
            else {
                return false;
            }
        }

        false
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
