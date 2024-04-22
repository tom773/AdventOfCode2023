#![allow(unused)]
#![allow(non_snake_case)]

use std::fs;

fn main() {
    
    let input = fs::read_to_string("src/4p1.txt").expect("Error reading file");
}

fn linefeed(input: &str){
    for (idx, ln) in input.lines().enumerate(){
        println!("{:?}", ln);
    }
}
