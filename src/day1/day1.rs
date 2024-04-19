use std::fs;

#[allow(dead_code)]
#[allow(unused)]

fn main() {
    
    let contents = fs::read_to_string("src/input.txt").expect("Error reading file");
    part1(contents);
}

fn part1(contents: String) -> i32 {
    let mut sum = 0;  
    for line in contents.lines() {
        let mut digits: Vec<i32> = Vec::new();
        for c in line.chars() {
            if c.is_digit(10) {
                digits.push(c.to_digit(10).unwrap() as i32);
            }
        }
        let first = digits.first().expect("Error getting first digit").to_string();
        let last = digits.last().expect("Error").to_string();
        let comb = first + &last;
        let intcomb = comb.parse::<i32>().expect("Error parsing");
        sum += intcomb; 
    }
    println!("Part 1: {}", sum);
    return sum;   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = String::from("1abc2\nqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet\n");
        assert_eq!(part1(contents), 142);
    }

}



