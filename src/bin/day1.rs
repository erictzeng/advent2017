extern crate clap;

use std::fs::File;
use std::io::Read;
use clap::{Arg, App};


fn match_digit(chars: &Vec<char>, index: usize, offset: usize) -> u32 {
    if chars[index] == chars[(index + offset) % chars.len()] {
        return chars[index].to_digit(10).expect("Non-digit in input");
    }
    return 0;
}


fn sum_matching_digits(chars: &Vec<char>, offset: usize) -> u32 {
    (0..chars.len()).map(|i| match_digit(&chars, i, offset)).sum()
}


fn main() {
    let matches = App::new("day1")
        .arg(Arg::with_name("input"))
        .get_matches();
    let mut file = File::open(matches.value_of("input").unwrap())
        .expect("Couldn't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Couldn't read file");
    let chars: Vec<_> = contents.chars().filter(|&char| char != '\n').collect();
    let sum1: u32 = sum_matching_digits(&chars, 1);
    let sum2: u32 = sum_matching_digits(&chars, chars.len() / 2);
    println!{"next digit sum: {}", sum1};
    println!{"half digit sum: {}", sum2};
}
