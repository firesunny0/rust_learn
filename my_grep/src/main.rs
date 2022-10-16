use std::env;
use std::fs;

struct Config {
    word: String,
    file_name: String,
}

impl Config {
    fn new(args: &[String]) -> () {}
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];

    let content = fs::read_to_string(file).unwrap_or_else(|err| {
        println!("Read file failed: {}", err);
        String::from("")
    });
    println!("{}", content);
}
