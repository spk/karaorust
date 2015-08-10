extern crate combine;
use combine::*;
use combine::primitives::Stream;

#[derive(PartialEq, Debug)]
pub struct Header {
    key: String,
    value: String,
}

#[derive(PartialEq, Debug)]
pub struct Lyric {
    duration: u32,
    text: String,
}

#[derive(PartialEq, Debug)]
pub struct Karaoke {
    pub headers: Vec<Header>,
    pub lyrics: Vec<Lyric>
}

fn header<I>(input: State<I>) -> ParseResult<Header, I>
where I: Stream<Item=char> {
    let lex_char = |c| char(c).skip(spaces());
    (lex_char('#')
     , many::<String, _>(alpha_num())
     , lex_char(':')
     , many::<String, _>(satisfy(|c| c != '\n')))
        .map(|(_, key, _, value)| {
            Header {
                key: key,
                value: value.trim_right().to_string()
            }
        }).parse_state(input)
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
     , many::<String, _>(satisfy(|c| c != '\n')))
        .map(|(_, _, _, _, duration, _, _, _, text)| {
            Lyric {
                duration: duration.parse::<u32>().unwrap() * 100,
                text: text.trim_right().to_string()
            }
        }).parse_state(input)
}

fn karaoke<I>(input: State<I>) -> ParseResult<Karaoke, I>
where I: Stream<Item=char> {
    (parser(header), newline(), parser(lyric), newline(), char('E'))
        .map(|(h, _, l, _, _)| {
            Karaoke {
                headers: vec![h],
                lyrics: vec![l]
            }
    }).parse_state(input)
}

#[cfg(not(test))]
fn main() {
}

#[test]
fn test_header_line() {
    let lines = "#TITLE:Natalie's Rap";
    let result = parser(header).parse(lines);
    let expr = Header { key: "TITLE".to_string(),
                        value: "Natalie's Rap".to_string()
    };
    assert_eq!(result, Ok((expr, "")));
}

#[test]
fn test_lyric_line() {
    let lines = ": 1 13 50 We're sitting here today with";
    let result = parser(lyric).parse(lines);
    let expr = Lyric { duration: 1300,
                       text: "We\'re sitting here today with".to_string()
    };
    assert_eq!(result, Ok((expr, "")));
}

#[test]
fn test_karaoke_simple() {
    let lines = "#TITLE:Natalie's Rap\n: 1 13 50 We're sitting here today with\nE";
    let result = parser(karaoke).parse(lines);
    let expr = Karaoke {
        headers: vec![Header { key: "TITLE".to_string(), value: "Natalie's Rap".to_string()}],
        lyrics: vec![Lyric { duration: 1300, text: "We\'re sitting here today with".to_string() }]
    };
    assert_eq!(result, Ok((expr, "")));
}
