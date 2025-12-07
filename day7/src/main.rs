use std::fs;


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut beams = Vec::new();
    let mut res = 0;

    for line in input.lines() {
        if beams.len() < line.len() {
            beams.resize(line.len(), false);
        }
        for (idx, ch) in line.chars().enumerate() {
            if ch == 'S' {
                beams[idx] = true;
            } else if ch == '^' && beams[idx] {
                res += 1;
                if idx > 0 {
                    beams[idx-1] = true;
                }
                if idx+1 < beams.len() {
                    beams[idx+1] = true;
                }
                beams[idx] = false;
            }
        }
    }
    println!("{res}");
}
