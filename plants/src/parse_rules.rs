use crate::pattern;


// Instantiate Pattern objects from a string.
pub fn parse_rules(data : &str) -> Vec<pattern::Pattern> {
    let mut result = Vec::new();

    for l in data.lines() {
        let split : Vec<&str> = l.split(':').collect();
        if split.len() != 2 {
            panic!("Invalid rule");
        }
        result.push(pattern::Pattern::new(split[0].chars().next().unwrap(),
                                 String::from(split[1])));
    }

    result
}
