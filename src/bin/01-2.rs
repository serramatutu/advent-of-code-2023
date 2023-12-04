use std::io;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let stdin = io::stdin();
    let mut sum = 0;

    let forward_pat = Regex::new(r"([1-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let backward_pat = Regex::new(r"([1-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();

    let value = HashMap::from([
        ("1", 1), ("one", 1),
        ("2", 2), ("two", 2),
        ("3", 3), ("three", 3),
        ("4", 4), ("four", 4),
        ("5", 5), ("five", 5),
        ("6", 6), ("six", 6),
        ("7", 7), ("seven", 7),
        ("8", 8), ("eight", 8),
        ("9", 9), ("nine", 9),
    ]);


    for line in stdin.lines() {
        let line_ok = line.unwrap();
        let first = &forward_pat.captures(&line_ok).unwrap()[0];


        let rev_line = line_ok.chars().rev().collect::<String>();
        let last_rev = &backward_pat.captures(&rev_line).unwrap()[0];
        let last = last_rev.chars().rev().collect::<String>();

        let num = 
            value.get(&first).unwrap() * 10 
            + value.get(&last.as_str()).unwrap();

        sum += num;

        println!("{}", num)
    }

    println!("The sum is {}", sum);
}