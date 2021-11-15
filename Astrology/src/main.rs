use fastrand;
use rayon::prelude::*;
use std::fs::OpenOptions;
use std::io::prelude::Write;
use std::time::Instant;

pub fn roll_opt(options: u8) -> (u8, u8) {
    let r = fastrand::u8(0..100);
    let val: u8;
    if r < 50 {
        val = 1;
    } else if r < 80 {
        val = 2;
    } else if r < 95 {
        val = 3
    } else if r < 99 {
        val = 4
    } else {
        val = 5
    }

    return (fastrand::u8(0..options), val);
}

pub fn is_roll_match(roll: (u8, u8), choice: (u8, u8)) -> bool {
    return choice.0 == roll.0 && choice.1 <= roll.1;
}

pub fn run_simulation(
    options: u8,
    choices: &[(u8, u8); 3],
    min_match: u8,
    full_cost: u128,
    single_cost: u128,
) -> (u128, u128) {
    let mut locked_choices: [bool; 3];
    let mut total_single_cost: u128 = 0;
    let mut total_full_cost: u128 = 0;

    loop {
        let mut locked_rolls: [bool; 3] = [false, false, false];
        locked_choices = [false, false, false];
        let rolls: [(u8, u8); 3] = [roll_opt(options), roll_opt(options), roll_opt(options)];
        total_full_cost += full_cost;
        let mut satisfied: u8 = 0;
        for choice_index in 0..choices.len() {
            if locked_choices[choice_index] {
                continue;
            }
            for roll_index in 0..rolls.len() {
                if locked_rolls[roll_index] {
                    continue;
                }
                if is_roll_match(rolls[roll_index], choices[choice_index]) {
                    satisfied += 1;
                    locked_rolls[roll_index] = true;
                    locked_choices[choice_index] = true;
                    break;
                }
            }
        }
        if satisfied >= min_match {
            if satisfied == choices.len() as u8 {
                return (total_full_cost, total_single_cost);
            }
            break;
        }
    }

    loop {
        let roll: (u8, u8) = roll_opt(options);
        total_single_cost += single_cost;
        let mut rarest_match: u8 = 0;
        let mut rarest_match_index: usize = 0;
        for choice_index in 0..choices.len() {
            if locked_choices[choice_index] {
                continue;
            }
            if is_roll_match(roll, choices[choice_index]) && choices[choice_index].1 > rarest_match
            {
                rarest_match_index = choice_index;
                rarest_match = choices[choice_index].1;
            }
        }
        if rarest_match > 0 {
            locked_choices[rarest_match_index] = true;
            if locked_choices[0] && locked_choices[1] && locked_choices[2] {
                return (total_full_cost, total_single_cost);
            }
        }
    }
}

pub fn run_simulation_total(
    options: u8,
    choices: &[(u8, u8); 3],
    min_match: u8,
    full_cost: u128,
    single_cost: u128,
) -> u128 {
    let result = run_simulation(options, choices, min_match, full_cost, single_cost);
    return result.0 + result.1;
}

pub fn simulate_multiple(
    options: u8,
    choices: &[(u8, u8); 3],
    min_match: u8,
    full_cost: u128,
    single_cost: u128,
    simulations: u128,
) -> (u128, u128) {
    let mut results_full: u128 = 0;
    let mut results_single: u128 = 0;

    for _ in 0..simulations {
        let sim_results = run_simulation(options, choices, min_match, full_cost, single_cost);
        results_full += sim_results.0;
        results_single += sim_results.1;
    }

    return (results_full / simulations, results_single / simulations);
}

pub fn simulate_multiple_total(
    options: u8,
    choices: &[(u8, u8); 3],
    min_match: u8,
    full_cost: u128,
    single_cost: u128,
    simulations: u128,
) -> u128 {
    return (0..simulations)
        .into_par_iter()
        .map(|_| run_simulation_total(options, choices, min_match, full_cost, single_cost))
        .sum::<u128>()
        / simulations;
}

pub fn get_permutations() -> Vec<[(u8, u8); 3]> {
    return Vec::from([[(0u8, 1u8), (0u8, 1u8), (0u8, 5u8)]]);
}

fn main() {
    const CHOICES: [[(u8, u8); 3]; 235] = [
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 5u8), (0u8, 5u8), (0u8, 5u8)],
        [(0u8, 5u8), (0u8, 5u8), (0u8, 5u8)],
        [(0u8, 5u8), (0u8, 5u8), (0u8, 5u8)],
        [(0u8, 5u8), (0u8, 5u8), (0u8, 5u8)],
        [(0u8, 5u8), (0u8, 5u8), (0u8, 5u8)],
        [(0u8, 5u8), (0u8, 5u8), (0u8, 5u8)],
        [(0u8, 5u8), (0u8, 5u8), (0u8, 5u8)],
        [(0u8, 5u8), (0u8, 5u8), (0u8, 5u8)],
        [(0u8, 5u8), (0u8, 5u8), (0u8, 5u8)],
        [(0u8, 5u8), (0u8, 5u8), (0u8, 5u8)],
        [(0u8, 5u8), (0u8, 5u8), (0u8, 5u8)],
        [(0u8, 5u8), (0u8, 5u8), (0u8, 5u8)],
        [(0u8, 5u8), (0u8, 5u8), (0u8, 5u8)],
        [(0u8, 5u8), (0u8, 5u8), (0u8, 5u8)],
        [(0u8, 5u8), (0u8, 5u8), (0u8, 5u8)],
        [(0u8, 5u8), (0u8, 5u8), (0u8, 5u8)],
        [(0u8, 5u8), (0u8, 5u8), (0u8, 5u8)],
        [(0u8, 5u8), (0u8, 5u8), (0u8, 5u8)],
        [(0u8, 5u8), (0u8, 5u8), (0u8, 5u8)],
        [(0u8, 5u8), (0u8, 5u8), (0u8, 5u8)],
        [(0u8, 5u8), (0u8, 5u8), (0u8, 5u8)],
        [(0u8, 5u8), (0u8, 5u8), (0u8, 5u8)],
        [(0u8, 5u8), (0u8, 5u8), (0u8, 5u8)],
        [(0u8, 5u8), (0u8, 5u8), (0u8, 5u8)],
        [(0u8, 5u8), (0u8, 5u8), (0u8, 5u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 5u8), (0u8, 5u8), (0u8, 5u8)],
        [(0u8, 5u8), (0u8, 5u8), (0u8, 5u8)],
        [(0u8, 5u8), (0u8, 5u8), (0u8, 5u8)],
        [(0u8, 5u8), (0u8, 5u8), (0u8, 5u8)],
        [(0u8, 5u8), (0u8, 5u8), (0u8, 5u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 1u8), (0u8, 1u8), (0u8, 1u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 2u8), (0u8, 2u8), (0u8, 2u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 5u8), (0u8, 5u8), (0u8, 5u8)],
    ];

    const MIN_CHOICES: [[(u8, u8); 3]; 55] = [
        [(0u8, 3u8), (1u8, 3u8), (2u8, 3u8)],
        [(0u8, 3u8), (1u8, 3u8), (2u8, 4u8)],
        [(0u8, 3u8), (1u8, 3u8), (2u8, 5u8)],
        [(0u8, 3u8), (1u8, 4u8), (2u8, 3u8)],
        [(0u8, 3u8), (1u8, 4u8), (2u8, 4u8)],
        [(0u8, 3u8), (1u8, 4u8), (2u8, 5u8)],
        [(0u8, 3u8), (1u8, 5u8), (2u8, 3u8)],
        [(0u8, 3u8), (1u8, 5u8), (2u8, 4u8)],
        [(0u8, 3u8), (1u8, 5u8), (2u8, 5u8)],
        [(0u8, 4u8), (1u8, 3u8), (2u8, 3u8)],
        [(0u8, 4u8), (1u8, 3u8), (2u8, 4u8)],
        [(0u8, 4u8), (1u8, 3u8), (2u8, 5u8)],
        [(0u8, 4u8), (1u8, 4u8), (2u8, 3u8)],
        [(0u8, 4u8), (1u8, 4u8), (2u8, 4u8)],
        [(0u8, 4u8), (1u8, 4u8), (2u8, 5u8)],
        [(0u8, 4u8), (1u8, 5u8), (2u8, 3u8)],
        [(0u8, 4u8), (1u8, 5u8), (2u8, 4u8)],
        [(0u8, 4u8), (1u8, 5u8), (2u8, 5u8)],
        [(0u8, 5u8), (1u8, 3u8), (2u8, 3u8)],
        [(0u8, 5u8), (1u8, 3u8), (2u8, 4u8)],
        [(0u8, 5u8), (1u8, 3u8), (2u8, 5u8)],
        [(0u8, 5u8), (1u8, 4u8), (2u8, 3u8)],
        [(0u8, 5u8), (1u8, 4u8), (2u8, 4u8)],
        [(0u8, 5u8), (1u8, 4u8), (2u8, 5u8)],
        [(0u8, 5u8), (1u8, 5u8), (2u8, 3u8)],
        [(0u8, 5u8), (1u8, 5u8), (2u8, 4u8)],
        [(0u8, 5u8), (1u8, 5u8), (2u8, 5u8)],
        [(0u8, 3u8), (0u8, 3u8), (1u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (1u8, 4u8)],
        [(0u8, 3u8), (0u8, 3u8), (1u8, 5u8)],
        [(0u8, 3u8), (0u8, 4u8), (1u8, 3u8)],
        [(0u8, 3u8), (0u8, 4u8), (1u8, 4u8)],
        [(0u8, 3u8), (0u8, 4u8), (1u8, 5u8)],
        [(0u8, 3u8), (0u8, 5u8), (1u8, 3u8)],
        [(0u8, 3u8), (0u8, 5u8), (1u8, 4u8)],
        [(0u8, 3u8), (0u8, 5u8), (1u8, 5u8)],
        [(0u8, 4u8), (0u8, 4u8), (1u8, 3u8)],
        [(0u8, 4u8), (0u8, 4u8), (1u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (1u8, 5u8)],
        [(0u8, 4u8), (0u8, 5u8), (1u8, 3u8)],
        [(0u8, 4u8), (0u8, 5u8), (1u8, 4u8)],
        [(0u8, 4u8), (0u8, 5u8), (1u8, 5u8)],
        [(0u8, 5u8), (0u8, 5u8), (1u8, 3u8)],
        [(0u8, 5u8), (0u8, 5u8), (1u8, 4u8)],
        [(0u8, 5u8), (0u8, 5u8), (1u8, 5u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 3u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 4u8)],
        [(0u8, 3u8), (0u8, 3u8), (0u8, 5u8)],
        [(0u8, 3u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 3u8), (0u8, 4u8), (0u8, 5u8)],
        [(0u8, 3u8), (0u8, 5u8), (0u8, 5u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 4u8)],
        [(0u8, 4u8), (0u8, 4u8), (0u8, 5u8)],
        [(0u8, 4u8), (0u8, 5u8), (0u8, 5u8)],
        [(0u8, 5u8), (0u8, 5u8), (0u8, 5u8)],
    ];

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("results.csv")
        .unwrap();
    if let Err(e) = writeln!(file, "Options,Choices,Match,Cost,Result") {
        eprintln!("Couldn't write to file: {}", e);
    }
    for total_options in 1u8..9u8 {
        for choices in CHOICES {
            let mut valid: bool = true;
            for choice in choices {
                if choice.0 >= total_options {
                    valid = false;
                    break;
                }
            }
            if !valid {
                continue;
            }
            let choices_string = format!(
                "{}{}{}{}{}{}",
                choices[0].0, choices[0].1, choices[1].0, choices[1].1, choices[2].0, choices[2].1
            );
            for single_cost in [15u128, 30u128] {
                for min_match in 0u8..4u8 {
                    let simulations: u128;
                    if min_match == 3 {
                        simulations = 1000;
                    } else if min_match == 2 {
                        simulations = 10000;
                    } else {
                        simulations = 100000;
                    }
                    let start = Instant::now();
                    let result = simulate_multiple_total(
                        total_options,
                        &choices,
                        min_match,
                        1,
                        single_cost,
                        simulations,
                    );
                    let execution_time = start.elapsed().as_millis();
                    let data = format!(
                        "{},{},{},{},{}",
                        total_options, choices_string, single_cost, min_match, result
                    );
                    println!("Data: < {} > | Elapsed: < {} >", data, execution_time);
                    if let Err(e) = writeln!(file, "{}", data) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                }
            }
        }
    }
}
