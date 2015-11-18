use std::collections::HashMap;

use combine::*;

use super::{ karaoke, read_karaoke_file, Karaoke, Lyric };

#[test]
fn test_header_line() {
    let lines = "#TITLE:Natalie's Rap\n#ARTIST:Natalie Portman\nE";
    let result = parser(karaoke).parse(lines);
    let mut header = HashMap::new();
    header.insert("TITLE".to_string(), "Natalie\'s Rap".to_string());
    header.insert("ARTIST".to_string(), "Natalie Portman".to_string());
    let expected = Karaoke {
        header: header,
        lyrics: vec![]
    };
    match result {
        Ok(result) => assert_eq!(result, (expected, "")),
        Err(err) => {
            println!("{}", err);
            assert!(false);
        }
    }
}

#[test]
fn test_karaoke_simple() {
    let lines = "#TITLE:Natalie's Rap\n: 1 13 50 We're sitting here today with\n- 13\nE";
    let result = parser(karaoke).parse(lines);
    let mut header = HashMap::new();
    header.insert("TITLE".to_string(), "Natalie\'s Rap".to_string());
    let expected = Karaoke {
        header: header,
        lyrics: vec![Lyric { duration: 1300, text: "We\'re sitting here today with".to_string() }]
    };
    match result {
        Ok(result) => assert_eq!(result, (expected, "")),
        Err(err) => {
            println!("{}", err);
            assert!(false);
        }
    }
}

#[test]
fn test_karaoke_complete() {
    let input = "data/Natalie_Portman_-_Natalies_Rap.txt";
    let buffer = read_karaoke_file(input);
    let text = from_iter(buffer.chars());
    match parser(karaoke).parse(text.clone()) {
        Ok((_, _)) => assert!(true),
        Err(err) => {
            println!("{}", err);
            assert!(false);
        }
    }
}
