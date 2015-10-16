extern crate karaorust;
extern crate combine;

use std::thread::sleep_ms;
use std::env;

use karaorust::*;
use combine::*;

#[cfg(not(test))]
fn main() {
    let args = env::args();
    let args_vec: Vec<String> = args.collect();
    let program = args_vec[0].clone();

    let input = input_from_args(env::args(), &program);
    let buffer = read_karaoke_file(&input, &program);
    let text = from_iter(buffer.chars());

    match parser(karaoke).parse(text.clone()) {
        Ok((k, _)) => {
            for (k, v) in k.header.iter() {
                println!("{} ~> {}", k, v);
            }
            for lyric in k.lyrics.iter() {
                println!("{}", lyric.text);
                sleep_ms(lyric.duration as u32);
            }
        },
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
}
