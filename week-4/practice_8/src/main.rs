fn main() {
    let mut x = 0;

    loop {
        println!("x = {}", x);
        x += 1;

        if x == 15 {
            break;
        }
    }

    println!("Loop exited at x = {}", x);
}
