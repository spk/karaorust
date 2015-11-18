extern crate karaorust;

use std::env;

use karaorust::{ parse_karaoke_file, input_from_args };

#[cfg(not(test))]
fn main() {
    let args = env::args();
    let args_vec: Vec<String> = args.collect();
    let program = args_vec[0].clone();

    let input = input_from_args(env::args(), &program);
    parse_karaoke_file(&input, &program);
}
