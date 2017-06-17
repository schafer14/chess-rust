extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::error::Error;


use bitboard;
use ai::ids;


pub fn test() {
    let path = Path::new("perftsuite.epd");
    let display = path.display();

    let re = self::regex::Regex::new(r"D5\s([0-9]+)").unwrap();

    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => println!("Starting:"),
    }

    let mut i = 0;
    for line in s.trim().split("\n").collect::<Vec<&str>>() {
        i = i + 1;
        let parts = line.trim().split("-").collect::<Vec<&str>>();

        let mut board = bitboard::BitBoard::from_fen(String::from(parts[0]));

        let expected = re.captures(parts.last().unwrap()).unwrap();
        let actual = ids::perft(board, 5).to_string();

        println!("Running test {:?}: {:?} - {:?}", i, &expected[1], actual);
        assert!(&expected[1] == actual);
    }


    println!("Done with Perft Testing");
}