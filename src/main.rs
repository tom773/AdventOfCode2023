use std::fs;

fn main() {
    
    let contents = fs::read_to_string("src/input.txt").expect("Error reading file");
    part1(&contents);
}

fn part1(contents: &str){
    contents.lines().for_each(|line| {
        println!("{}", line);
    });
}

