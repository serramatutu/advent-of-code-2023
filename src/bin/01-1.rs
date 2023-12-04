use std::io;


fn main() {
    let stdin = io::stdin();
    let mut sum = 0;

    for line in stdin.lines() {
        let line_ok = line.unwrap();

        let first_idx = line_ok.find(|c: char| c >= '0' && c <= '9').expect("first num char not found");
        let last_idx = line_ok.rfind(|c: char| c >= '0' && c <= '9').expect("last num char not found");

        let mut num_str = String::with_capacity(2);
        num_str.push(line_ok.chars().nth(first_idx).unwrap());
        num_str.push(line_ok.chars().nth(last_idx).unwrap());

        let num = num_str.parse::<i32>().unwrap();
        sum += num;
    }

    println!("The sum is {}", sum);
}