use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("Compressing {}", file_path);

}
