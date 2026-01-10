use aoc_lib::read_lines;
use std::collections::HashMap;

type TargetState = u32;
type Button = u32;
type Cache = HashMap<(u32, usize), usize>;

fn get_target_states(lines: &[String]) -> Vec<TargetState> {
    lines
        .iter()
        .map(|l| {
            let state_string = l.split(']').next().unwrap().split('[').next_back().unwrap();
            let mut state: TargetState = 0;
            for (i, value) in state_string.chars().enumerate() {
                if value == '#' {
                    state ^= 1 << i;
                }
            }
            state
        })
        .collect()
}

fn get_buttons(lines: &[String]) -> Vec<Vec<Button>> {
    lines
        .iter()
        .map(|l| {
            let buttons_strings: Vec<&str> = l.split_whitespace().collect();
            buttons_strings
                .iter()
                .take(buttons_strings.len() - 1)
                .skip(1)
                .map(|b| {
                    let positions_string = &b[1..b.len() - 1];
                    let positions: Vec<usize> = positions_string
                        .split(',')
                        .map(|p| p.parse().unwrap())
                        .collect();
                    let mut button: Button = 0;
                    positions.iter().for_each(|p| {
                        button |= 1 << p;
                    });
                    button
                })
                .collect::<Vec<Button>>()
        })
        .collect()
}

fn press_button(state: u32, button: Button) -> u32 {
    state ^ button
}

fn solve(
    cache: &mut Cache,
    state: u32,
    target_state: TargetState,
    buttons: &[Button],
    presses: usize,
) -> usize {
    if cache.contains_key(&(state, presses)) {
        *cache.get(&(state, presses)).unwrap()
    } else if state == target_state {
        0
    } else if presses > 10 {
        usize::MAX
    } else {
        let mut best: usize = usize::MAX;

        for button in buttons {
            let new_state = press_button(state, *button);
            let result = 1 + solve(cache, new_state, target_state, buttons, presses + 1);

            if result < best {
                best = result;
            }
        }

        cache.insert((state, presses), best);
        best
    }
}

fn get_button_presses_counts(
    target_states: Vec<TargetState>,
    buttons: Vec<Vec<Button>>,
) -> Vec<usize> {
    target_states
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let mut cache: Cache = HashMap::new();
            solve(&mut cache, 0, *s, buttons.get(i).unwrap(), 0)
        })
        .collect()
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");

    let target_states = get_target_states(&lines);
    let buttons = get_buttons(&lines);

    let button_presses = get_button_presses_counts(target_states, buttons);
    println!(
        "Button presses sum: {}",
        button_presses.iter().sum::<usize>()
    );
}
