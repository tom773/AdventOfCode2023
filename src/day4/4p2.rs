#![allow(unused)]
#![allow(non_snake_case)]

use std::fs;
use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    
    let input = fs::read_to_string("src/4p1.txt").expect("Error reading file");
    let total = linefeed(&input);
    println!("{}", total);
    
}

fn linefeed(input: &str) -> usize{
    
    let mut copies = vec![1; input.split('\n').count()-1];

    for (i, j) in input.split('\n').enumerate() {
        
        if j == "" {continue}
        
        let (_, line) = j.split_once(": ").unwrap();
        
        let (ours, winners) = line.split_once(" | ").unwrap();
        
        let ours = ours.split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        
        let won = winners.split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .filter(|d| ours.contains(d))
            .count();
        
        for l in 0..won {
            copies[i+l+1] += copies[i];
        }
    }
    
    return copies.iter().sum();
}
