use std::io;

fn main() {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    println!("\x1b[35mMagic Compiler\x1b[m \x1b[34m#version: {}\x1b[m \n", VERSION);

    let mut buffer = String::new();
    let coord_x: i32;
    let coord_y: i32;
    let radius : i32;
    let mut color = String::new();

    io::stdin().read_line(&mut buffer).expect("Incorrect string.");
    coord_x = buffer.trim().parse().expect("cannnot parse.");
    io::stdin().read_line(&mut buffer).expect("Incorrect string.");
    coord_y = buffer.trim().parse().expect("cannnot parse.");
    io::stdin().read_line(&mut buffer).expect("Incorrect string.");
    radius = buffer.trim().parse().expect("cannnot parse.");
    io::stdin().read_line(&mut color).expect("Incorrect string.");

    // Algorism
    println!("<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\" />", coord_x, coord_y, radius, color);
}
