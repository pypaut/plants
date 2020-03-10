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

        let mut left = ' ';
        let mut pat = split[0].chars().next().unwrap();
        let mut right = ' ';

        // Check for context sensitivity
        let split2 : Vec<&str> = split[0].split('<').collect();
        if split2.len() == 2 {
            if split2[0].chars().next() != None {
                left = split2[0].chars().next().unwrap();
            }
            let split3 : Vec<&str>  = split2.clone()[1].split('>').collect();
            pat = split3[0].chars().next().unwrap();
            if split3[1].chars().next() != None {
                right = split3[1].chars().next().unwrap();
            }
        }

        result.push(pattern::Pattern::new(
                        pat,
                        String::from(split[i]),
                        p.into(),
                        left,
                        right,
            )
        );
    }

    result
}
