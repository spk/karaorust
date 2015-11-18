use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::process;
use std::thread::sleep_ms;

use combine::primitives::{ from_iter, Parser, ParseError, ParseResult, State, Stream };
use combine::combinator::{ many, parser, satisfy, Expected, Skip, skip_many, skip_many1, token,
                           FnParser, ParserExt };
use combine::char::{ char, digit, space, spaces, Spaces, string, newline, alpha_num };


#[derive(PartialEq, Debug)]
pub struct Lyric {
    pub duration: u32,
    pub text: String,
}

#[derive(PartialEq, Debug)]
pub struct Karaoke {
    pub header: HashMap<String, String>,
    pub lyrics: Vec<Lyric>
}

fn header<I>(input: State<I>) -> ParseResult<(String, String), I>
where I: Stream<Item=char> {
    let lex_char = |c| char(c).skip(spaces());
    (lex_char('#')
     , many::<String, _>(alpha_num())
     , lex_char(':')
     , many::<String, _>(satisfy(|c| c != '\n'))
     , newline())
        .map(|(_, key, _, value, _)| (key, value.trim_right().to_string()))
        .expected("header")
        .parse_state(input)
}

fn headers<I>(input: State<I>) -> ParseResult<HashMap<String, String>, I>
where I: Stream<Item=char> {
    many(parser(header))
        .parse_state(input)
}

fn lyric<I>(input: State<I>) -> ParseResult<Lyric, I>
where I: Stream<Item=char> {
    (char(':')
     , spaces()
     , many::<String, _>(digit())
     , spaces()
     , many::<String, _>(digit())
     , spaces()
     , many::<String, _>(digit())
     , spaces()
     , many::<String, _>(satisfy(|c| c != '\n'))
     , newline())
        .map(|(_, _, _, _, duration, _, _, _, text, _)| {
            Lyric {
                duration: duration.parse::<u32>().unwrap() * 100,
                text: text.trim_right().to_string()
            }
        })
    .expected("lyric")
    .parse_state(input)
}

fn split<I>(input: State<I>) -> ParseResult<(), I>
where I: Stream<Item=char> {
    let split = (token('-'), skip_many(satisfy(|c| c != '\n'))).map(|_| ());
    skip_many(skip_many1(space()).or(split))
        .parse_state(input)
}

pub fn karaoke<I>(input: State<I>) -> ParseResult<Karaoke, I>
where I: Stream<Item=char> {
    (parser(headers), many(parser(lyric).skip(parser(split))), char('E'))
        .map(|(h, l, _)| {
            Karaoke {
                header: h,
                lyrics: l
            }
    }).parse_state(input)
}

pub fn read_karaoke_file(input: &str) -> String {
    let path = Path::new(&input);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(err) => {
            println!("Couldn't open {}: {}", display, Error::description(&err));
            process::exit(1);
        },
        Ok(file) => file,
    };

    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Err(err) => {
            println!("Couldn't read {}: {}", display, Error::description(&err));
            process::exit(1);
        },
        Ok(_) => println!("Starting karaoke with {}", display),
    }
    buffer
}

pub fn parse_karaoke_file(input: &str) {
    let buffer = read_karaoke_file(&input);
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
