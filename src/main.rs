#![allow(unused)]
#![allow(non_snake_case)]

use std::fs;
use std::collections::HashMap;


fn main() {
    
    let mut total = 0;
    let input = fs::read_to_string("src/7p1.txt").expect("Error reading file");
    for (idx, ln) in input.lines().enumerate(){
        let bid = linefeed(ln, idx);
        total += bid*bid;
    }
    println!("Total: {}", total);
}

fn linefeed(input: &str, ln: usize) -> i32{
    
    let count = 0;

    let (hand, bid) = input.split_once(" ").unwrap();
    let bid_ = bid.parse().unwrap();
    println!("{}: {}", hand, bid);
    return bid_; 
}
