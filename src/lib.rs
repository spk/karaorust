#![crate_name = "karaorust"]

extern crate combine;

mod parser;
pub use parser::{ karaoke, parse_karaoke_file, read_karaoke_file, Karaoke, Lyric };

#[cfg(test)]
mod tests;
