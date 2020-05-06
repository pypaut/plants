use std::f64::consts::PI;
use std::{env, fs};
use std::fs::File;
use std::error::Error;
use std::io::Write;
use std::convert::TryInto;

mod engine;
mod matrix4;
mod mesh;
mod turtle;
mod vector3;
mod object;


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

    // Available colors
    let mut colors = Vec::new();
    colors.push((90.0, 35.0, 35.0));    // 5a2323, brown
    colors.push((118.0, 156.0, 35.0));  // 769c23, green

    // Generate segments
    let nb_colors : i64 = (&colors).len().try_into().unwrap();
    let (segments, leaves) = engine::read_str(&in_str,
                                      dist, angle * (PI / 180.0), (min_d, max_d),
                                              reason_d,
                                              nb_colors);

    // Generate & print geometry
    let meshes = engine::gen_geometry(segments, leaves, nb_colors);  // Meshes are indexed with their color index

    for i in 0..nb_colors {  // nb_colors == nb_meshes
        let j : usize = i as usize;

        // Create .obj text
        let out_str = meshes[j].clone().get_str();

        // Open .obj file
        let tmp_output = output.clone() + &j.to_string() + ".obj";
        let mut file = match File::create(&tmp_output) {
            Err(why) => panic!("Couldn't create {}: {}", output, why.description()),
            Ok(file) => file,
        };

        // Write .obj file
        match file.write_all(out_str.as_bytes()) {
            Err(why) => panic!("Couldn't write to {}: {}", tmp_output, why.description()),
            Ok(_) => println!("Successfully wrote to {}", tmp_output),
        };
    }

    // // Open final .mtl file
    // let mut file = match File::create(&output) {
    //     Err(why) => panic!("Couldn't create {}: {}", output, why.description()),
    //     Ok(file) => file,
    // };

    // // Write final .mtl file
    // match file.write_all(out_str.as_bytes()) {
    //     Err(why) => panic!("Couldn't write to {}: {}", output, why.description()),
    //     Ok(_) => println!("Successfully wrote to {}", output),
    // };
}
