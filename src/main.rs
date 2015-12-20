extern crate karaorust;

use std::env;
use std::process;

use karaorust::{ parse_karaoke_file };

#[cfg(not(test))]
fn print_usage(program: &str) {
    let brief = format!("Usage: {} FILE", program);
    println!("{}", &brief);
}

#[cfg(not(test))]
fn check_argv(mut argv: env::Args) -> Result<String, String> {
    argv.nth(1)
        .ok_or("Please give at least one argument".to_owned())
}

#[cfg(not(test))]
fn input_from_args(args: env::Args, program: &str) -> String {
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

#[cfg(not(test))]
fn main() {
    let args = env::args();
    let args_vec: Vec<String> = args.collect();
    let program = args_vec[0].clone();

    let input = input_from_args(env::args(), &program);

    parse_karaoke_file(&input);
}
