use std::io;

use regex::Regex;

fn is_number_or_period(c: char) -> bool {
    (c > '0' && c < '9') || c == '.'
}

fn is_part(x: usize, y: usize, len: usize, grid: &Vec<Vec<char>>) -> bool {
    for x in x-1..x+len+1 {
        if !is_number_or_period(grid[y-1][x]) || !is_number_or_period(grid[y+1][x]) {
            return true
        }
    }

    !is_number_or_period(grid[y][x-1]) || !is_number_or_period(grid[y][x+len])
}

fn main() {
    let stdin = io::stdin();
    let mut sum = 0u32;

    let num_pat = Regex::new(r"\d+").unwrap();

    let mut first_line = String::new();
    stdin.read_line(&mut first_line).unwrap();
    first_line.pop();

    let y_guard: Vec<char> = std::iter::repeat(".").take(first_line.len() + 2).collect::<String>().chars().collect();
    let mut grid = vec![
        y_guard.clone(), 
        format!(".{}.", first_line).chars().collect()
    ];

    stdin.lines()
        .map(|l| format!(".{}.", l.unwrap()))
        .for_each(|l| grid.push(l.chars().collect::<Vec<char>>()));
    grid.push(y_guard);

    for (y, line) in grid.iter().enumerate() {
        let line_str = line.iter().collect::<String>();

        for num_match in num_pat.find_iter(&line_str) {
            let x = num_match.start();
            let len = num_match.len();

            if is_part(x, y, len, &grid) {
                let num = num_match.as_str().parse::<u32>().unwrap();
                sum += num;
            }
        }
    }

    println!("sum: {}", sum);
}