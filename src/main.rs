#![allow(dead_code)]

fn main() {
    println!("Hello, World!");
}

// Rust pattern matching
// match (origin, target)

fn miles_kilometers(_val: i32) -> i32 {
    32
}

fn convert_units(value: f64, origin: &str, target: &str) -> f64 {
    match (origin, target) {
        ("m", "km") | ("km", "m") => {
            if origin == "m" {
                value / 1000.0
            } else {
                value * 1000.0
            }
        }
        // Add more conversions here as needed
        _ => panic!("Unsupported conversion"),
    }
}
