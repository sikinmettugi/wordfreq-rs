
// Word frequency counter
// Inspired by "Exercises in Programming Style"

use std::env;
use std::fs::File;
use std::io::Read;
use std::str;

struct WordCount {
    word: String,
    count: u32,
}

impl WordCount {
    fn get_count(&self) -> u32 {
        self.count
    }

    fn increment(&mut self) {
        self.count += 1
    }
}


fn main() {
    let stop_words_filename = "stop_words.txt";
    let args: Vec<String> = env::args().collect();
    println!("Hello, world!");
    println!("args: {:?}", args);
    // What to do:
    // 1. Make a stop_word list from file
    // 2. Read input file
    // 3. Lower cases, remove stop_words
    // 4. Count

    //let mut stop_words: Vec<String> = Vec::new();
    let mut stop_words_file: File = File::open(stop_words_filename)
                                        .expect("Stop words file doesn't exists");
    // Read whole stop words file, then append to vector
    let mut contents: String = String::new();
    stop_words_file.read_to_string(&mut contents)
                    .expect("Failed to read stop words file");

    //println!("{}", contents);
    let mut stop_words: Vec<String> = contents.split(",")
                                            .filter(|s| s.len() != 0)
                                            .map(|s| s.to_string().to_lowercase())
                                            .collect();
    let mut ascii_lowers: Vec<String> = (0..26).map(|x| ((x + 'a' as u8) as char).to_string())
                                                .collect::<Vec<String>>();
    stop_words.append(&mut ascii_lowers);
    for w in stop_words {
        println!("{}", w);
    }


}
