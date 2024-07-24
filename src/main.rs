mod vblike;

use vblike::is_match;
fn main() {
    println!("{}", is_match("a*a", "abba"));
}
