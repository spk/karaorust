use super::*;
use combine::*;

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
