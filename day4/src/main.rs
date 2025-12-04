use std::fs;


fn main() {
    let input = fs::read_to_string("input1.txt").unwrap();
    let input : Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let mut res = 0;

    for (row_idx, row) in input.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            if *cell == '@' && count_adjacent(&input, row_idx, col_idx) < 4 {
                res += 1;
            }
        }
    }
    println!("{res}");
}

fn count_adjacent(input: &Vec<Vec<char>>, row: usize, col: usize) -> u32 {
    let mut res = 0;
    let start_row = row.checked_sub(1).unwrap_or(0);
    let start_col = col.checked_sub(1).unwrap_or(0);
    for r in start_row ..= row+1 {
        for c in start_col ..= col+1 {
            if r == row && c == col { continue; }
            if let Some(cell) = input.get(r).and_then(|r| r.get(c)) {
                if *cell == '@' {
                    res += 1;
                }
            }
        }
    }
    res
}
