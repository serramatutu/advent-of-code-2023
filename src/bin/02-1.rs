use std::io;

fn main() {
    let stdin = io::stdin();
    let mut id_sum = 0;

    'game_loop: for (game_id, line) in stdin.lines().enumerate() {
        let line_ok = line.unwrap();

        let (_, samples_str) = line_ok.split_once(": ").unwrap();
        let sample_strs: Vec<&str> = samples_str.split("; ").collect();

        for sample_str in sample_strs {
            let color_strs: Vec<&str> = sample_str.split(", ").collect();

            for color_str in color_strs {
                let (num_str, color) = color_str.split_once(" ").unwrap();

                let num = num_str.parse::<u8>().unwrap();

                if (color == "red" && num > 12) || (color == "green" && num > 13) || (color == "blue" && num > 14) {
                    continue 'game_loop;
                }
            }
        }

        id_sum += game_id + 1;
    }

    println!("id_sum: {}", id_sum);
}