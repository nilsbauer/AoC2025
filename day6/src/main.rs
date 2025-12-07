use std::fs;


fn main() {
    let input = fs::read_to_string("input1.txt").unwrap();
    let mut nums : Vec<Vec<u128>> = Vec::new();
    let mut res = 0;

    for line in input.lines() {
        for (idx, col) in line.split_ascii_whitespace().enumerate() {
            if col.trim() == "+" {
                res += nums[idx].iter().fold(0, |acc, x| acc + x);
                continue;
            }
            if col.trim() == "*" {
                res += nums[idx].iter().fold(1, |acc, x| acc * x);
                continue;
            }
            let col : u128 = col.parse().unwrap();
            if let Some(col_list) = nums.get_mut(idx) {
                col_list.push(col);
            } else {
                let col_list = vec![col];
                nums.push(col_list);
            }
        }
    }

    println!("{res}");
}
