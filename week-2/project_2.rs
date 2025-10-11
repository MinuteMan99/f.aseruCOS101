fn main() {
   let a = 2.0 * 450_000.0 + 1.0 * 1_500_000.0 + 3.0 * 750_000.0 + 3.0 * 2_850_000.0 + 1.0 * 250_000.0;
   let q = 2.0 + 1.0 + 3.0 + 3.0 + 1.0;
   let avg = a/q;
   println!("The sum and average of the sales is {}", avg);
}