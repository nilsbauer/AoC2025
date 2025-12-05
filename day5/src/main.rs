use std::fs;


fn main() {
    let input = fs::read_to_string("input1.txt").unwrap();
    let mut ranges = Vec::new();
    let mut ranges_read = false;
    let mut res = 0;

    for line in input.lines() {
        if !ranges_read {
            if let Some((start, end)) = line.split_once('-') {
                let start : u64 = start.parse().unwrap();
                let end : u64 = end.parse().unwrap();
                ranges.push(start..=end);
            } else {
                ranges_read = true;
            }
        } else {
            let num : u64 = line.parse().unwrap();
            for range in &ranges {
                if range.contains(&num) {
                    res += 1;
                    break;
                }
            }
        }
    }
    println!("------------");
    println!("{res}");
}
