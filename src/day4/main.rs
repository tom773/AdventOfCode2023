#![allow(unused)]
#![allow(non_snake_case)]

use std::fs;

fn main() {
    let mut total = 0; 
    let contents = fs::read_to_string("src/4p1.txt").expect("Error reading file");
    contents.lines().for_each(|line| {
        let (nums, winners) = linefeed(line);
        let toadd = getMatches(nums, winners);
        total += toadd;
    });
    println!("{:?}", total);
}

fn linefeed(line: &str) -> (Vec<i32>, Vec<i32>){
    let mut split = vec![];
    let mut nums: Vec<i32> = vec![];
    let mut winners: Vec<i32>= vec![];

    line.split("|").for_each(|word| {
        split.push(word);
    });
    split[1] = split[1].trim();
    split[0] = split[0].split(":").nth(1).expect("Error splitting").trim();
   
    for i in split[0].split(" "){
        let t = i.trim(); 
        if t == "" || t == " "{
            continue;
        }
        nums.push(t.parse().expect("Error parsing"));
    }
    for i in split[1].split(" "){
        let t = i.trim();
        if t == "" || t == " "{
            continue;
        }
        winners.push(t.parse().expect("Error parsing"));
    }
    
    return (nums, winners);
}

fn getMatches(nums: Vec<i32>, winners: Vec<i32>) -> i32{
    let mut matches: Vec<i32> = vec![];
    let mut total = 0;
    for num in nums{
        for winner in &winners{
            if num == *winner{
                matches.push(num);
            }
        }
    }
    if matches.len() == 0{
        return total;
    } else {
        let base: i32 = 2;
        let matcheslen = matches.len()-1;
        total += base.pow(matcheslen as u32);
    }
    return total;
}
