use crate::pattern;


// Instantiate Pattern objects from a string.
pub fn parse_rules(data : &str) -> Vec<pattern::Pattern> {
    let mut result = Vec::new();

    for l in data.lines() {
        let split : Vec<&str> = l.split(':').collect();
        if split.len() != 2 && split.len() != 3 {
            panic!("Invalid rule");
        }
        let mut p = 1.0;
        let mut i = 1;
        if split.len() == 3 {
           p = split[1].parse::<f32>().unwrap();
           i = 2;
        }
        result.push(pattern::Pattern::new(
                        split[0].chars().next().unwrap(),
                        String::from(split[i]),
                        p.into()
            )
        );
    }

    result
}
