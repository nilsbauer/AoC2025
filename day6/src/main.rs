use std::fs;


fn main() {
    let input = fs::read_to_string("input1.txt").unwrap();
    let mut nums : Vec<u128> = Vec::new();
    let mut res = 0;

    for line in input.lines() {
        if nums.len() < line.len() {
            nums.resize(line.len(), 0);
        }
        for (idx, col) in line.chars().enumerate() {
            if let Some(digit) = col.to_digit(10) {
                if let Some(col_val) = nums.get_mut(idx) {
                    *col_val = *col_val * 10 + u128::from(digit);
                }
            } else if col == '+' {
                let mut i = idx;
                let mut term = 0;
                loop {
                    if i >= nums.len() || nums[i] == 0 {
                        break;
                    } else {
                        term += nums[i];
                    }
                    i += 1;
                }
                res += term;
            } else if col == '*' {
                let mut i = idx;
                let mut term = 1;
                loop {
                    if i >= nums.len() || nums[i] == 0 {
                        break;
                    } else {
                        term *= nums[i];
                    }
                    i += 1;
                }
                res += term;
            }
        }
    }


    println!("{res}");
}
