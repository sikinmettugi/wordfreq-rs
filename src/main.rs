
// Word frequency counter
// Inspired by "Exercises in Programming Style"

use std::env;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Hello, world!");
    println!("args: {:?}", args);
    // What to do:
    // 1. Make a stop_word list from file
    // 2. Read input file
    // 3. Lower cases, remove stop_words
    // 4. Count

}
