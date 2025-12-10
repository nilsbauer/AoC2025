use std::fs;

use regex::Regex;


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let re = Regex::new(r"(?x)
\[(?P<indicator>[\.\#]+)\]\s*
(?P<wiring>[(),0-9\ ]+)\s*
\{(?P<joltage>[\{\}\,0-9]+)\}
    ").unwrap();
    let mut res = 0;

    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let indicator = &caps["indicator"];
        let wiring = &caps["wiring"].trim();
        let _joltage = &caps["joltage"];

        let buttons = ButtonState::parse(wiring, indicator.len());

        if let Some(num_presses) = buttons.solve(indicator) {
            res += num_presses;
        } else {
            panic!("couldn't solve");
        }
    }
    println!("{res}");
}

#[derive(Debug,Clone)]
struct ButtonState {
    buttons: Vec<Vec<u32>>,
    pressed: Vec<bool>,
    light_num: usize,
}

impl ButtonState {
    fn indicators(&self) -> String {
        let mut ret = vec!['.'; self.light_num];
        for (button, pressed) in self.buttons.iter().zip(self.pressed.iter()) {
            if *pressed {
                for light in button {
                    let light = usize::try_from(*light).unwrap();
                    ret[light] = if ret[light] == '.' { '#' } else { '.' };
                }
            }
        }
        ret.iter().collect()
    }

    fn solve(&self, final_state: &str) -> Option<u32> {
        for num_presses in 0..=self.buttons.len() {
            if self.solve_with_n_presses(final_state, num_presses, 0) {
                return Some(num_presses.try_into().unwrap());
            }
        }
        None
    }

    fn solve_with_n_presses(&self, final_state: &str, remaining_presses: usize, start_idx: usize) -> bool {
        if remaining_presses == 0 {
            return self.indicators() == final_state;
        }
        for next_press in start_idx..self.buttons.len() {
            let mut next_state = self.clone();
            next_state.pressed[next_press] = true;
            if next_state.solve_with_n_presses(final_state, remaining_presses-1, next_press+1) {
                return true;
            }
        }
        false
    }

    //fn print_pressed_state(&self) -> String {
    //    self.pressed.iter().map(|b| {
    //        if *b { 'X' } else { '-' }
    //    }).collect()
    //}

    fn parse(s: &str, light_num: usize) -> Self {
        let buttons: Vec<_> =
            s.split(' ')
            .map(|b| {
                b.trim_matches(&['(', ')'])
                 .split(',')
                 .map(|d| d.parse::<u32>().unwrap())
                 .collect::<Vec<_>>()
            }).collect();
        let pressed = vec![false; buttons.len()];
        ButtonState { buttons, pressed, light_num }
    }
}
