fn main() {
    let fullname = " Pan-Atlantic University ";
    println!("Name: {}", fullname);
    println!("Before trim: '{}'", fullname.len());
    
    println!("After trim: '{}'", fullname.trim().len());
    println!("length is {}", fullname.trim().len());
}