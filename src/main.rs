pub mod parser;
pub mod eval;
pub mod errors;
pub mod builtins;
pub mod repl;

use repl::repl;

fn main() {
    /* let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print!("You must provide filecode.");
    }

    let content = fs::read_to_string(args[1].clone()).expect("Cannot read file for some reasons."); */
    repl();
}
