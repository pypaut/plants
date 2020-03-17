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
    let mut args = env::args();
    args.next();
    let input = args.next()
        .expect("usage: ./graph3d input output [angle] [dist] [reason] [min_d] [max_d]")
        .clone();
    println!("{}", input);
    let output = args.next()
        .expect("Not enough arguments.")
        .clone();
    let angle = args.next()
        .unwrap_or(String::from("22.5"))
        .parse::<f64>()
        .expect("Invalid value for angle");
    let dist = args.next()
        .unwrap_or(String::from("1.0"))
        .parse::<f64>()
        .expect("Invalid value for distance.");
    let reason_d = args.next()
        .unwrap_or(String::from("0.8"))
        .parse::<f64>().expect("Invalid value for reason");
    let min_d = args.next()
        .unwrap_or(String::from("0.1"))
        .parse::<f64>().expect("Invalid value for min_d.");
    let max_d = args.next()
        .unwrap_or(String::from("0.5"))
        .parse::<f64>().expect("Invalid value for max_d.");

    // Read file and get string
    let in_str = fs::read_to_string(input)
        .expect("Failed reading file.");

    // Generate segments
    let (segments, leaves) = engine::read_str(&in_str,
                                      dist, angle * (PI / 180.0), (min_d, max_d),
                                              reason_d);

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
