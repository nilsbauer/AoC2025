use std::fs;
use std::str::FromStr;
use std::sync::LazyLock;

use regex::Regex;


fn main() {
    let input = fs::read_to_string("input1.txt").unwrap();
    let mut res = 0;

    for line in input.lines() {
        let mut state : ButtonState = line.parse().unwrap();
        if let Some(num_presses) = state.solve() {
            println!("solved in {num_presses} presses");
            res += num_presses;
        } else {
            panic!("couldn't solve");
        }
    }
    println!("{res}");
}

#[derive(Debug,Clone)]
struct ButtonState {
    buttons: Vec<Vec<usize>>,
    presses: Vec<u32>,
    joltage_targets: Vec<u32>,
    current_joltage: Vec<u32>,
}

impl ButtonState {
    fn determine_next_button(&self) -> Option<usize> {
        for button_size in (0 ..= self.buttons[0].len()).rev() {
            if let Some((ret, _, _)) =
                self.buttons.iter()
                    .zip(self.calc_possible_button_presses())
                    .enumerate()
                    .map(|(idx, (button, poss_presses))| (idx, button, poss_presses))
                    .filter(|(_, b, _)| b.len() == button_size)
                    .max_by_key(|(_, _, p)| *p)
            {
                if ret > 0 {
                    return Some(ret);
                }
            }
        }
        None
    }

    fn solve(&mut self) -> Option<u32> {
        let mut ret = 0;
        while let Some(next_button) = self.determine_next_button() {
            println!("trying to press button {next_button}");
            self.press_button(next_button);
            println!("joltages: {:?}", self.current_joltage);
            ret += 1;
        }
        if self.cmp_joltage() {
            Some(ret)
        } else {
            None
        }
    }

    fn press_button(&mut self, idx: usize) {
        self.presses[idx] += 1;
        for joltage in &self.buttons[idx] {
            self.current_joltage[*joltage] += 1;
        }
    }

    fn cmp_joltage(&self) -> bool {
        let mut joltages = vec![0; self.joltage_targets.len()];
        for (button_joltages, presses) in self.buttons.iter().zip(self.presses.iter()) {
            for button_joltage in button_joltages {
                joltages[*button_joltage] += presses;
            }
        }
        joltages.iter().zip(self.joltage_targets.iter()).all(|(joltage, target)| *joltage == *target)
    }

    //fn exceeds_joltage(&self) -> bool {
    //    let mut joltages = vec![0; self.joltage_targets.len()];
    //    for (button_joltages, presses) in self.buttons.iter().zip(self.presses.iter()) {
    //        for button_joltage in button_joltages {
    //            joltages[*button_joltage] += presses;
    //        }
    //    }
    //    joltages.iter().zip(self.joltage_targets.iter()).any(|(joltage, target)| *joltage > *target)
    //}

    fn calc_possible_button_presses(&self) -> Vec<u32> {
        let ret = self.buttons.iter()
            .map(|b| b.iter()
                .map(|j| self.joltage_targets[*j] - self.current_joltage[*j])
                .min()
                .unwrap())
            .collect();
        println!("possible: {ret:?}");
        ret
    }
}

impl FromStr for ButtonState {
    type Err = ButtonStateParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?x)
\[(?P<indicator>[\.\#]+)\]\s*
(?P<wiring>[(),0-9\ ]+)\s*
\{(?P<joltage>[\{\}\,0-9]+)\}
    ").unwrap());

        let caps = RE.captures(s).unwrap();
        let _indicator = &caps["indicator"];
        let wiring = &caps["wiring"].trim();
        let joltage = &caps["joltage"];

        let mut buttons: Vec<_> =
            wiring.split(' ')
            .map(|b| {
                b.trim_matches(&['(', ')'])
                 .split(',')
                 .map(|d| d.parse::<usize>().unwrap())
                 .collect::<Vec<_>>()
            })
        .collect();
        buttons.sort_by_key(|b| b.len());
        buttons.reverse();
        let presses = vec![0; buttons.len()];

        let joltage_targets : Vec<_> = joltage.split(',').map(|j| j.parse().unwrap()).collect();
        let current_joltage = vec![0; joltage_targets.len()];
        println!("buttons: {buttons:?}");
        Ok(ButtonState { buttons, presses, joltage_targets, current_joltage })
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ButtonStateParseError {
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    //fn tr() {
    //    let buttons = vec![[3], [2], [1, 3], [2, 3], [0, 2], [0, 1]];
    //    let possible = vec![7, 4, 5, 4, 3, 3];
    //    let button_size = 1;
    //    self.buttons.iter()
    //        .zip(self.calc_possible_button_presses())
    //        .enumerate()
    //        .map(|(idx, (button, poss_presses))| (idx, button, poss_presses))
    //        .filter(|(_, b, _)| b.len() == button_size)
    //        .max_by_key(|(_, _, p)| *p)
    //}

    #[test]
    fn tr2() {
        let data = vec![vec![0,2,3,4],vec![2,3],vec![0,4],vec![0,1,2],vec![1,2,3,4]];
        get_next_idx(&data, Vec::new());
    }

    fn get_next_idx(data: &Vec<Vec<usize>>, factors: Vec<i32>) {
        if factors.len() < 5 {
            for factor in -1..=2 {
                let mut factors = factors.clone();
                factors.push(factor);
                get_next_idx(data, factors);
            }
        } else {
            for (idx, factor) in factors.iter().enumerate() {
                let term = match factor {
                    -1 => format!("- x{idx} "),
                    0  => format!("+0x{idx} "),
                    1  => format!("+ x{idx} "),
                    f  => format!("+ {f}x{idx} "),
                };
                print!("{term}");
            }
            print!(" = ");
            let mut result = vec![0; 5];
            for (factor, data) in factors.iter().zip(data.iter()) {
                for d in data {
                    result[*d] += factor;
                }
            }
            for res in &result {
                print!("{res:>2} ");
            }
            let num0 = result.iter().fold(0, |acc, x| if *x == 0 { acc + 1} else { acc });
            if num0 == 4 {
                print!("HIERHIERHIER");
            }
            println!("   | {}", num0);
        }
    }
}
