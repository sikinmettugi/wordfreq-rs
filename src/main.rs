// Word frequency counter
// Inspired by "Exercises in Programming Style"
// Written by: Chang-Jae Lee

use std::env;
use std::fs::File;
use std::io::Read;
use std::str;
use std::iter::FromIterator;
use std::collections::{HashSet, HashMap};
use std::cmp;

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
    //println!("Hello, world!");
    //println!("args: {:?}", args);

    if args.len() < 2 {
        println!("Usage: {} input_filename", args[0]);
        return
    }

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
                                            .map(|s: &str| s.to_string().to_lowercase())
                                            .collect();
    let mut ascii_lowers: Vec<String> = (0..26)
                                         .map(|x| ((x + 'a' as u8) as char).to_string())
                                         .collect::<Vec<String>>();
    stop_words.append(&mut ascii_lowers);
    let stop_words_set: HashSet<String> = HashSet::from_iter(stop_words);
//    for w in stop_words {
//        println!("{}", w);
//    }

    // Read input file
    let mut input_file: File = File::open(&args[1])
                                .expect( format!("Cannot open input file {}", args[1]).as_ref());

    let mut contents: String = String::new();
    input_file.read_to_string(&mut contents)
              .expect("Failed to read input file");

    //println!("contents: \n{}", contents);
    let words: Vec<String> = contents.chars()
                                      .map(|c: char| if c.is_alphanumeric() { c } else { ' ' }).collect::<String>()
                                      .split_whitespace()
                                      .map(|s: &str| s.to_string().to_lowercase())
                                      // Remove stop words
                                      .filter(|s: &String| !stop_words_set.contains(s))
                                      .collect();

    //println!("{:?}", words);

    // Count occurrences
    let mut word_counts: HashMap<String, usize> = HashMap::new();

    for word in words {
        if let Some(count) = word_counts.get_mut(&word) {
            *count += 1;
            continue;
        }
        word_counts.insert(word, 1);
    }
    //println!("{:?}", word_counts);

    let mut sortable_words: Vec<(&str, usize)> = word_counts.iter().map(|(w, c)| (w.as_str(), *c)).collect();
    sortable_words.sort_by_key(|a| a.0);
    sortable_words.sort_by(|a, b| b.1.cmp(&a.1));

    //println!("{:?}", sortable_words);

    // Print first 25 elements
    let iter_range = cmp::min(sortable_words.len(), 25);

    for occ in &sortable_words[0..iter_range] {
        println!("{}: {}", occ.0, occ.1);
    }
}
