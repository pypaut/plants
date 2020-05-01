use std::collections::HashMap;
use crate::pattern::Pattern;


#[derive(Debug)]
pub struct IterCtx {
    pub ignored: String,//ignored characters for context test
    pub axiom: String,//axiom used to initialize
    pub n_iter: usize,//number of iterations
    pub define: HashMap<String, f32>,//saved constants
    pub include: HashMap<String, String>,//included predefined shapes
    pub patterns: Vec<Pattern>
}

impl IterCtx {
    pub fn to_light_ctx(&self) -> LightCtx {
        LightCtx{ignored: self.ignored.clone(), define: self.define.clone()}
    }
}

pub struct LightCtx {
    pub ignored: String,
    pub define: HashMap<String, f32>
}
