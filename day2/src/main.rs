use std::collections::HashSet;
use std::fs;


fn main() {
    let input = fs::read_to_string("input1.txt").unwrap();
    let mut res = 0;

    for range in input.split(',') {
        let (start, end) = range.trim().split_once('-').unwrap();
        let start: u64 = start.parse().unwrap();
        let end: u64 = end.parse().unwrap();
        let mut numbers = HashSet::new();

        for nu in start..=end {
            let num = nu.to_string();
            for patlen in 1..=(num.len() / 2) {
                if num.len() % patlen != 0 { continue; }
                let pattern = &num[0..patlen];
                if is_pattern(&num, pattern) && !numbers.contains(&nu) {
                    println!("found {num}");
                    res += nu;
                    numbers.insert(nu);
                }
            }
        }
    }
    println!("res is {res}");
}

fn is_pattern(num: &str, pat: &str) -> bool {
    let patlen = pat.len();
    for rep in 1..(num.len() / patlen) {
        let start = rep * patlen;
        let end = start + patlen;
        if &num[start..end] != pat {
            return false;
        }
    }
    return true;
}
