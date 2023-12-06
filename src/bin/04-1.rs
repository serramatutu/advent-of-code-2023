use std::io;

fn str_to_arr(str: &str) -> Vec<u32> {
    str.split(" ")
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

fn main() {
    let stdin = io::stdin();
    let mut sum = 0u32;

    for line in stdin.lines() {
        let line_ok = line.unwrap();

        let (_, nums) = line_ok.split_once(": ").unwrap();
        let (winning_str, mine_str) = nums.split_once(" | ").unwrap();

        let mut winning = str_to_arr(winning_str);
        let mut mine = str_to_arr(mine_str);

        winning.sort();
        mine.sort();

        let mut count = 0;
        let mut i = 0;
        for num in mine {
            while i < winning.len() && winning[i] < num {
                i += 1
            }

            if i == winning.len() {
                break
            }

            if winning[i] == num {
                count += 1
            }
        }

        if count > 0 {
            sum += 1 << count - 1;
        }
    }

    println!("sum: {}", sum);
}