use std::fs;


fn main() {
    let input = fs::read_to_string("input1.txt").unwrap();
    let mut pos = 50;
    let mut res = 0;

    for line in input.lines() {
        let (dir, dist) = line.split_at(1);
        let mut dist : i64 = dist.parse().unwrap();
        while dist > 99 {
            dist -= 100;
            res += 1;
        }
        let oldpos = pos;
        match dir {
            "L" => {
                pos = pos - dist;
                eprintln!("moving left {dist}, new pos: {pos}");
            },
            "R" => {
                pos = pos + dist;
                eprintln!("moving right {dist}, new pos: {pos}");
            },
            _   => panic!("Invalid Direction: {dir}"),
        };
        while pos > 99 {
            pos -= 100;
            if pos != 0 {
                res += 1;
                eprintln!("inc1 after {line}");
            }
        }
        while pos < 0 {
            pos += 100;
            if pos != 0 && oldpos != 0 {
                res += 1;
                eprintln!("inc2 after {line}");
            }
        }
        if pos == 0 && oldpos != 0 {
            res += 1;
            eprintln!("inc0 after {line}");
        }
    }
    println!("res: {res}");
}
