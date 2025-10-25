use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Enter value of a:");
    io::stdin().read_line(&mut a).unwrap();

    println!("Enter value of b:");
    io::stdin().read_line(&mut b).unwrap();

    println!("Enter value of c:");
    io::stdin().read_line(&mut c).unwrap();

    let a: f64 = a.trim().parse().unwrap();
    let b: f64 = b.trim().parse().unwrap();
    let c: f64 = c.trim().parse().unwrap();

    let d = b * b - 4.0 * a * c;

    if d > 0.0 {
        let r1 = (-b + d.sqrt()) / (2.0 * a);
        let r2 = (-b - d.sqrt()) / (2.0 * a);
        println!("Two distinct real roots: {} and {}", r1, r2);
    } else if d == 0.0 {
        let r = -b / (2.0 * a);
        println!("One real root: {}", r);
    } else {
        println!("No real roots.");
    }
    }
