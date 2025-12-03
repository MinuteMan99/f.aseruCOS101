use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");
    println!("This is the file {}", contents);
}