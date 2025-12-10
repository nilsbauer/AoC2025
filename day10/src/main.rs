use std::fs;
use std::str::FromStr;
use std::sync::LazyLock;

use regex::Regex;


fn main() {
    let input = fs::read_to_string("input1.txt").unwrap();
    let mut res = 0;

    for line in input.lines() {
        let state : ButtonState = line.parse().unwrap();
        if let Some(num_presses) = state.solve() {
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
    possible_button_presses: Vec<u32>,
}

impl ButtonState {
    fn determine_next_button(&self) -> Option<usize> {
        let mut biggest_button_size = self.buttons[0].len();
        for button_size in (0 ..= self.buttons[0].len()).rev() {

            if let Some(ret, _, _) =
                self.buttons.iter()
                    .zip(self.possible_button_presses)
                    .enumerate()
                    .map(|(idx, (button, poss_presses))| (idx, button, poss_presses))
                    .filter(|(_, b, _)| b.len() == button_size)
                    .max_by_key(|(_, _, p)| p)
            {
                return Some(ret);
            }
        }
        None
    }

    fn solve(&self) -> Option<u32> {
        for num_presses in 0..13 {
            println!("trying to solve with {num_presses} presses");
            if self.solve_with_n_presses(num_presses) {
                println!("found a solution with {num_presses} presses");
                return Some(num_presses.try_into().unwrap());
            }
        }
        None
    }

    fn solve_with_n_presses(&self, remaining_presses: usize) -> bool {
        if remaining_presses == 0 {
            return self.cmp_joltage()
        }
        if self.exceeds_joltage() {
            return false;
        }
        for next_press in 0..self.buttons.len() {
            let mut next_state = self.clone();
            next_state.presses[next_press] += 1;
            if next_state.solve_with_n_presses(remaining_presses-1) {
                return true;
            }
        }
        false
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

    fn exceeds_joltage(&self) -> bool {
        let mut joltages = vec![0; self.joltage_targets.len()];
        for (button_joltages, presses) in self.buttons.iter().zip(self.presses.iter()) {
            for button_joltage in button_joltages {
                joltages[*button_joltage] += presses;
            }
        }
        joltages.iter().zip(self.joltage_targets.iter()).any(|(joltage, target)| *joltage > *target)
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
        let presses = vec![0; buttons.len()];

        let joltage_targets = joltage.split(',').map(|j| j.parse().unwrap()).collect();
        let current_joltage = vec![0; joltage_targets.len()];
        let possible_button_presses = calc_possible_button_presses(buttons, joltage_targets);
        Ok(ButtonState { buttons, presses, joltage_targets, current_joltage, possible_button_presses  })
    }
}

fn calc_possible_button_presses(buttons: &Vec<Vec<usize>>, joltage_targets: &Vec<u32>) -> Vec<u32> {
    buttons.map(|b| b.map(|j| joltage_target[j]).min().unwrap()).collect()
}

#[derive(Debug, PartialEq, Eq)]
enum ButtonStateParseError {
}
