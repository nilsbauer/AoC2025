use std::fs;

const DIGIT_NUM: usize = 12;

fn main() {
    let input = fs::read_to_string("input1.txt").unwrap();
    let input : Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let mut res = 0;

    for line in input {
        let mut last_idx = 0;
        let mut line_res : u64 = 0;
        for digit_idx in 0..DIGIT_NUM {
            let pot_digits = &line[last_idx .. line.len() - DIGIT_NUM + digit_idx + 1];
            let mut current_digit = None;
            for (idx, digit) in pot_digits.iter().enumerate() {
                current_digit = current_digit.map_or(Some((idx, digit)), |(i, dig)|
                    if dig < digit {
                        Some((idx, digit))
                    } else {
                        Some((i, dig))
                    }
                );
            }
            let (idx, digit) = current_digit.unwrap();
            last_idx += idx + 1;
            line_res = line_res * 10 + u64::from(digit.to_digit(10).unwrap());
        }
        println!("{line_res}");
        res += line_res;
    }
    println!("--------------");
    println!("{res}");
}
