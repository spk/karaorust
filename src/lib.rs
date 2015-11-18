#![crate_name = "karaorust"]

use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::process;
use std::env;
use std::thread::sleep_ms;

use karaorust::parser;

fn check_argv(mut argv: env::Args) -> Result<String, String> {
    argv.nth(1)
        .ok_or("Please give at least one argument".to_owned())
}

pub fn input_from_args(args: env::Args, program: &str) -> String {
    let input = match check_argv(args) {
        Ok(n) => n,
        Err(err) => {
            println!("Error: {}", err);
            print_usage(&program);
            process::exit(1);
        },
    };
    input
}

pub fn read_karaoke_file(input: &str, program: &str) -> String {
    let path = Path::new(&input);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(err) => {
            println!("Couldn't open {}: {}", display, Error::description(&err));
            print_usage(&program);
            process::exit(1);
        },
        Ok(file) => file,
    };

    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Err(err) => {
            println!("Couldn't read {}: {}", display, Error::description(&err));
            print_usage(&program);
            process::exit(1);
        },
        Ok(_) => println!("Starting karaoke with {}", display),
    }
    buffer
}

pub fn parse_karaoke_file(input: &str, program: &str) {
    let buffer = read_karaoke_file(&input, &program);
    let text = from_iter(buffer.chars());

    match parser(karaoke).parse(text.clone()) {
        Ok((k, _)) => {
            for (k, v) in k.header.iter() {
                println!("{} ~> {}", k, v);
            }
            for lyric in k.lyrics.iter() {
                println!("{}", lyric.text);
                sleep_ms(lyric.duration);
            }
        },
        Err(err) => {
            println!("Couldn't read file: {}", Error::description(&err));
            process::exit(1);
        }
    };
}

#[cfg(test)]
mod tests;
