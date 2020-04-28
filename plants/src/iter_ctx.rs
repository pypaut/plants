use std::collections::HashMap;


#[derive(Debug)]
pub struct IterCtx {
    pub ignored: String,//ignored characters for context test
    pub axion: String,//axiom used to initialize
    pub n_iter: usize,//number of iterations
    pub define: HashMap<String, f32>,//saved constants
    pub include: Vec<String>//included predefined shapes
}
