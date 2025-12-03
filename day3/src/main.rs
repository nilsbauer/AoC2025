use std::fs;


fn main() {
    let input = fs::read_to_string("input1.txt").unwrap();
    let input : Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let mut res = 0;

    for line in input {
        let mut first_digit = None;
        let pot_first_digit = &line[0..line.len() - 1];
        for (idx, digit) in pot_first_digit.iter().enumerate() {
            first_digit = first_digit.map_or(Some((idx, digit)), |(i, dig)|
                if dig < digit {
                    Some((idx, digit))
                } else {
                    Some((i, dig))
                }
            );
        }
        let (first_idx, first_digit) = first_digit.unwrap();

        let pot_second_digit = &line[first_idx+1..line.len()];
        let mut second_digit = None;
        for digit in pot_second_digit {
            second_digit = second_digit.map_or(Some(digit), |d|
                if d < digit {
                    Some(digit)
                } else {
                    Some(d)
                }
            );
        }
        let second_digit = second_digit.unwrap();
        let line_res = first_digit.to_digit(10).unwrap() * 10 + second_digit.to_digit(10).unwrap();
        res += line_res;

        println!("{}", line_res);
    }
    println!("--------------");
    println!("{res}");
}
