use good_lp::{Expression, Solution, SolverModel, default_solver, variable, variables};
use std::collections::HashSet;

use crate::{solver::solver::Solver, utils::file_reader::FileReader};
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

static LIGHT_IND_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[([^\]]+)\]").unwrap());
static BUTTON_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\(([^)]+)\)").unwrap());
static VOLTAGE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\{([^}]+)\}").unwrap());

type Switch = Vec<char>;
type Button = HashSet<usize>;
type Voltage = Vec<usize>;

#[derive(Debug)]
struct SwitchState {
    light_ind: Switch,
    buttons: Vec<Button>,
    voltages: Voltage,
}

impl SwitchState {
    fn calculate_target_set(&self) -> HashSet<usize> {
        self.light_ind
            .iter()
            .enumerate()
            .filter(|&(_, &ch)| ch == '#') // Only positions that should be 'on'
            .map(|(i, _)| i)
            .collect()
    }
}

fn xor_multiple_sets(sets: &[&HashSet<usize>]) -> HashSet<usize> {
    if sets.is_empty() {
        return HashSet::new();
    }

    let mut result = sets[0].clone();
    for set in &sets[1..] {
        result = result.symmetric_difference(set).cloned().collect();
    }
    result
}

fn parse_switch_state(line: &str) -> SwitchState {
    let parse_usize_list =
        |s: &str| -> Vec<usize> { s.split(",").map(|x| x.trim().parse().unwrap()).collect() };

    SwitchState {
        light_ind: LIGHT_IND_RE
            .captures(line)
            .map(|caps| caps[1].chars().collect())
            .unwrap_or_default(),
        buttons: BUTTON_RE
            .captures_iter(line)
            .map(|caps| {
                parse_usize_list(&caps[1])
                    .into_iter()
                    .collect::<HashSet<usize>>()
            })
            .collect(),
        voltages: VOLTAGE_RE
            .captures(line)
            .map(|caps| parse_usize_list(&caps[1]))
            .unwrap_or_default(),
    }
}

pub struct Day10 {
    configurations: Vec<SwitchState>,
}

impl Solver<i32> for Day10 {
    fn new<R: FileReader>(reader: &R, file_path: &str) -> Result<Self, String> {
        let data = reader.read_file(file_path)?;
        let configurations: Vec<SwitchState> = data.split("\n").map(parse_switch_state).collect();
        Ok(Day10 { configurations })
    }

    fn part_one_solution(&mut self) -> i32 {
        // part 1 figure out lowest number of button presses
        // I'm thinking you neded to check the light indicator - its an XOR problem
        // .##. = 0110
        // you're looking for a XOR that matches to 0110
        // given two buttons if the XOR = 0110 it matches, so 0,2 and 0,1 = 0110
        let mut minimum_button_presses: Vec<usize> = vec![];
        for config in self.configurations.iter_mut() {
            let target = config.calculate_target_set();
            'num_button_loop: for num_buttons in 1..=config.buttons.len() {
                for combo in (0..config.buttons.len()).combinations(num_buttons) {
                    let button_sets: Vec<&HashSet<usize>> =
                        combo.iter().map(|&idx| &config.buttons[idx]).collect();
                    let combined = xor_multiple_sets(button_sets.as_slice());
                    if combined == target {
                        minimum_button_presses.push(num_buttons);
                        break 'num_button_loop;
                    }
                }
            }
        }
        minimum_button_presses
            .iter()
            .map(|&x| x as i32)
            .sum::<i32>()
    }

    fn part_two_solution(&mut self) -> i32 {
        let mut minimum_button_presses_voltage: Vec<usize> = vec![];
        for config in self.configurations.iter() {
            let mut vars = variables!();
            let button_counts: Vec<_> = (0..config.buttons.len())
                .map(|_| vars.add(variable().min(0).integer()))
                .collect();
            let objective: Expression = button_counts
                .iter()
                .fold(Expression::default(), |acc, &var| acc + var);
            let mut model = vars.minimise(objective).using(default_solver);
            for (voltage_idx, &target) in config.voltages.iter().enumerate() {
                let mut expr = Expression::default();
                for (button_idx, button) in config.buttons.iter().enumerate() {
                    if button.contains(&voltage_idx) {
                        expr = expr + button_counts[button_idx];
                    }
                }
                model = model.with(expr.eq(target as f64));
            }

            let solution = model.solve().unwrap();
            let total_presses: usize = button_counts
                .iter()
                .map(|&var| solution.value(var) as usize)
                .sum();

            minimum_button_presses_voltage.push(total_presses);
        }

        minimum_button_presses_voltage
            .iter()
            .map(|&x| x as i32)
            .sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::file_reader::StdFileReader;

    use super::*;

    #[test]
    fn test_part_one() {
        let reader = StdFileReader;
        let mut solver = Day10::from_test_path(&reader, 2025, 10).expect("Failed to load input");
        assert_eq!(solver.part_one_solution(), 7)
    }

    #[test]
    fn test_part_two() {
        let reader = StdFileReader;
        let mut solver = Day10::from_test_path(&reader, 2025, 10).expect("Failed to load input");
        assert_eq!(solver.part_two_solution(), 33)
    }
}
