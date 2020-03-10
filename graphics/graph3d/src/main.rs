use std::f64::consts::PI;
use std::{env, fs};
use std::fs::File;
use std::error::Error;
use std::io::Write;

mod engine;
mod mesh;
mod turtle;
mod vector3;


fn main() {
    // Parse arguments
    let args : Vec<String> = env::args().collect();
    let input = args[1].clone();
    let output = args[2].clone();
    let angle = args[3].parse::<f64>().expect("Failed while parsing angle.");
    let dist = args[4].parse::<f64>().expect("Failed while parsing distance.");

    // Read file and get string
    let in_str = fs::read_to_string(input)
        .expect("Failed reading file.");

    // Generate segments
    let (segments, leaves) = engine::read_str(&in_str,
                                      dist, angle * (PI / 180.0));

    // Generate & print geometry
    let mesh = engine::gen_geometry(segments, leaves);
    let out_str = mesh.get_str();

    let mut file = match File::create(&output) {
        Err(why) => panic!("Couldn't create {}: {}", output, why.description()),
        Ok(file) => file,
    };

    match file.write_all(out_str.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", output, why.description()),
        Ok(_) => println!("Successfully wrote to {}", output),
    }
}
