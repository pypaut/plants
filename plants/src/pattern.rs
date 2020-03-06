use rand::{thread_rng, Rng};


pub struct Pattern {
    pub pattern : char,       // Initial character
    pub replacement : String, // Replacement string
    pub p : f32               // Replacement probability
}

impl Pattern {
    pub fn new(pat : char, r : String, p : f32) -> Pattern {
        Pattern{pattern: pat, replacement: r, p: p}
    }

    pub fn test(&self, c : char, p : f32) -> bool {
        let mut rng = thread_rng();
        if rng.gen_bool(p.into()) {
            c == self.pattern
        }
        else {
            false
        }
    }
}
