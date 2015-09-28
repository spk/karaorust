use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::{Path};
use std::thread::sleep_ms;
use std::env;

#[cfg(not(test))]
fn print_usage(program: &str) {
    let brief = format!("Usage: {} FILE", program);
    print!("{}", &brief);
}

#[cfg(not(test))]
fn check_argv(mut argv: env::Args) -> Result<String, String> {
    argv.nth(1)
        .ok_or("Please give at least one argument".to_owned())
}

#[cfg(not(test))]
fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let input = match check_argv(env::args()) {
        Ok(n) => n,
        Err(err) => {
            println!("Error: {}", err);
            print_usage(&program);
            return;
        },
    };

    let file = match File::open(&Path::new(&input)) {
        Ok(file) => file,
        Err(err)  => {
            println!("Error: {}", err);
            print_usage(&program);
            return;
        },
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
