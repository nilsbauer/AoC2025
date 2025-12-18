use std::fs;

use regex::Regex;


fn main() {
    let re = Regex::new(r"(?x)
\[(?P<indicator>[\.\#]+)\]\s*
(?P<wiring>[(),0-9\ ]+)\s*
\{(?P<joltage>[\{\}\,0-9]+)\}
    ").unwrap();
    let input = fs::read_to_string("input2.txt").unwrap();


    for line in input.lines() {
        println!();
        let caps = re.captures(line).unwrap();
        let _indicator = &caps["indicator"];
        let wiring = &caps["wiring"].trim();
        let joltages = &caps["joltage"];

        let buttons : Vec<_> = wiring
            .split(' ')
            .map(|b|
                b.trim_matches(&['(', ')'])
                 .split(',')
                 .map(|d| d.parse::<i32>().unwrap())
                 .collect::<Vec<_>>()
            )
            .collect();
        let joltages : Vec<_> = joltages.split(',').map(|d| d.parse::<i32>().unwrap()).collect();

        let mut matrix = Vec::new();
        for (joltage_idx, joltage) in joltages.iter().enumerate() {
            let mut row = vec![0; buttons.len()];
            for (button_idx, button) in buttons.iter().enumerate() {
                for wire in button {
                    if *wire == joltage_idx.try_into().unwrap() {
                        row[button_idx] = 1;
                    }
                }
            }
            row.push(*joltage);
            matrix.push(row);
        }
        println!("Matrix:\n");
        print_matrix(&matrix);

        let mut target_matrix = Vec::new();

        // find defining row for each variable
        for col_idx in 0..buttons.len() {
            if let Some(row_idx) = find_row_for_variable(&matrix, col_idx) {
                println!("\nrow for column {}: {}", col_idx+1, row_idx+1);
                let mut row = matrix.swap_remove(row_idx);
                let factor = row[col_idx];
                for value in row.iter_mut() {
                    *value /= factor;
                }

                println!("\nnormalize remaining rows:");
                for matrix_row in matrix.iter_mut() {
                    if matrix_row[col_idx] != 0 {
                        let factor = matrix_row[col_idx];
                        for (col_idx, value) in matrix_row.iter_mut().enumerate() {
                            *value -= row[col_idx] * factor;
                        }
                    }
                }
                print_matrix(&matrix);

                target_matrix.push(Some(row));
            } else {
                println!("no row found for column {}", col_idx+1);
                target_matrix.push(None);
            }
            println!("\nsaved rows:");
            print_option_matrix(&target_matrix);
        }

        let additional_vars = 
        println!("------------------------------------------");
    }
}

fn find_row_for_variable(matrix: &Vec<Vec<i32>>, column: usize) -> Option<usize> {
    for (row_idx, row) in matrix.iter().enumerate() {
        if row[column] != 0 {
            return Some(row_idx);
        }
    }
    None
}

fn print_matrix(matrix: &Vec<Vec<i32>>) {
    for row in matrix {
        print!("| ");
        for value in row {
            print!("{value:>3} ");
        }
        println!("|");
    }
}

fn print_option_matrix(matrix: &Vec<Option<Vec<i32>>>) {
    let mut row_len = None;
    for row in matrix {
        if let Some(row) = row {
            row_len.get_or_insert(row.len());
            print!("| ");
            for value in row {
                print!("{value:>3} ");
            }
            println!("|");
        } else {
            let row_len = row_len.get_or_insert(find_row_len(matrix).unwrap_or(0));
            println!("| {}|", "    ".repeat(*row_len));
        }
    }
}

fn find_row_len(matrix: &Vec<Option<Vec<i32>>>) -> Option<usize> {
    for row in matrix {
        if let Some(row) = row {
            return Some(row.len());
        }
    }
    None
}
