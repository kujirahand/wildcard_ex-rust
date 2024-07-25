mod ex;
use ex::is_match;
fn main() {
    // This command line tests whether $2 matches $1.
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} [pattern] [text]", args[0]);
        return;
    }
    let pattern = &args[1];
    let text = &args[2];
    if is_match(pattern, text) {
        println!("True");
    } else {
        println!("False");
    }
}
