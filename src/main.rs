use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::{Path};
use std::thread::sleep_ms;
use std::env;

#[cfg(not(test))]
fn main() {
    if let Some(input) = env::args().nth(1) {
        let file = match File::open(&Path::new(&input)) {
            Ok(file) => file,
            Err(..)  => panic!("bim"),
        };
        let reader = BufReader::new(&file);
        let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

        for line in lines.iter() {
            if line.starts_with('#') {
                let metadata: Vec<&str> = line.split(":").collect();
                println!("{} ~> {}", metadata[0], metadata[1]);
            } else if line.starts_with(':') {
                let row: Vec<&str> = line.split(' ').collect();
                let duration = row[2].parse::<u32>().unwrap() * 100;
                let text = &row[4 .. row.len()].join(" ");;
                println!("{}", text);
                sleep_ms(duration);
            } else if line.starts_with('-') {
                println!("{}", "");
            }
        }
    }
}
