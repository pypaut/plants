pub struct Pattern {
    pub pattern : char,
    pub replacement : String
}

impl Pattern {
    pub fn new(p : char, r : String) -> Pattern {
        Pattern{pattern: p, replacement: r}
    }

    pub fn test(&self, c : char) -> bool {
        c == self.pattern
    }
}
