use std::fs;


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut beams = Vec::new();

    for line in input.lines() {
        if beams.len() < line.len() {
            beams.resize(line.len(), 0u128);
        }
        let mut next_beams = beams.clone();
        for (idx, ch) in line.chars().enumerate() {
            if ch == 'S' {
                next_beams[idx] = 1;
            } else if ch == '^' && beams[idx] > 0 {
                if idx > 0 {
                    next_beams[idx-1] += beams[idx];
                }
                if idx+1 < beams.len() {
                    next_beams[idx+1] += beams[idx];
                }
                next_beams[idx] = 0;
            }
        }
        beams = next_beams;
    }
    let res = beams.iter().fold(0, |acc, x| acc + x);
    println!("{res}");
}
