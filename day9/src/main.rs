use std::fs;


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut points = Vec::new();
    let mut biggest_area = 0;

    for line in input.lines() {
        let (x, y) = line.split_once(',').unwrap();
        let x : i64 = x.parse().unwrap();
        let y : i64 = y.parse().unwrap();
        points.push((x, y));
    }
    for (idx1, (x1, y1)) in points.iter().enumerate() {
        for (x2, y2) in points.iter().skip(idx1+1) {
            let area = i64::abs((x1 - x2 + 1) * (y1 - y2 + 1));
            if area > biggest_area {
                biggest_area = area;
            }
        }
    }
    println!("{biggest_area}");
}
