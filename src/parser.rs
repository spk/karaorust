extern crate combine;

use combine::*;
use combine::primitives::Stream;

use std::collections::HashMap;

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

fn print_usage(program: &str) {
    let brief = format!("Usage: {} FILE", program);
    println!("{}", &brief);
}
