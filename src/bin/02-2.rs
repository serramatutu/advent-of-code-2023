use std::io;
use std::cmp;

fn main() {
    let stdin = io::stdin();
    let mut power_sum = 0;

    for line in stdin.lines() {
        let line_ok = line.unwrap();

        let (_, samples_str) = line_ok.split_once(": ").unwrap();
        let sample_strs: Vec<&str> = samples_str.split("; ").collect();

        let (mut r, mut g, mut b) = (0u32, 0u32, 0u32);

        for sample_str in sample_strs {
            let color_strs: Vec<&str> = sample_str.split(", ").collect();

            for color_str in color_strs {
                let (num_str, color) = color_str.split_once(" ").unwrap();

                let num = num_str.parse::<u32>().unwrap();

                let var: &mut u32 = match color {
                    "red" => &mut r,
                    "green" => &mut g,
                    "blue" => &mut b,
                    _ => panic!("Invalid color")
                };

                *var = cmp::max(num, *var);
            }
        }

        let power:u32 = r * g * b;
        power_sum += power;
    }

    println!("power_sum: {}", power_sum);
}