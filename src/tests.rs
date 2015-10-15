use super::*;

use std::collections::HashMap;
use combine::*;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

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
    let mut data = String::new();
    File::open(&Path::new(&"data/Natalie_Portman_-_Natalies_Rap.txt"))
        .and_then(|mut file| file.read_to_string(&mut data))
        .unwrap();
    let text = from_iter(data.chars());
    match parser(karaoke).parse(text.clone()) {
        Ok((Karaoke(_), _)) => (),
        Ok(_) => assert!(false),
        Err(err) => {
            println!("{}", err);
            assert!(false);
        }
    }
}
