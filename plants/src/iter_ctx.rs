use std::collections::HashMap;
use crate::pattern::Pattern;


#[derive(Debug)]
pub struct IterCtx {
    pub ignored: String,//ignored characters for context test
    pub axiom: String,//axiom used to initialize
    pub n_iter: usize,//number of iterations
    pub define: HashMap<String, f32>,//saved constants
    pub include: HashMap<String, String>,//included predefined shapes
    pub patterns: Vec<Pattern>,
    pub objects: HashMap<String, String>
}

impl IterCtx {
    pub fn to_light_ctx(&self) -> LightCtx {
        LightCtx{ignored: self.ignored.clone(), define: self.define.clone()}
    }

    pub fn get_object_header(&self, rule_set: &String, folder: &String) -> String {
        let mut result = String::new();
        for (obj, file) in &self.objects {
            let mut tmp = rule_set.clone();
            tmp.push_str(obj);
            tmp.push(' ');
            tmp.push_str(format!("{}/{}", folder, file).as_str());
            tmp.push(' ');
            result.push_str(&tmp);
        }
        result
    }
}

pub struct LightCtx {
    pub ignored: String,
    pub define: HashMap<String, f32>
}
