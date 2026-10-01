use std::env;

fn main() {
    let name = env::args()
        .nth(1)
        .unwrap_or_else(|| "Developer".to_string());

    println!("Hello {} from Rust!", name);
}
