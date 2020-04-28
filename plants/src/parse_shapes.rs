use std::collections::HashMap;


pub fn parse_shapes(s: &str) -> HashMap<String, String> {
    let mut map : HashMap<String, String> = HashMap::new();
    for line in s.lines() {
        let split: Vec<&str> = line.split(':').collect();
        map.insert(
            String::from(split[0]),  // Shape name
            String::from(split[1])   // Shape value
        );
    }

    map
}
