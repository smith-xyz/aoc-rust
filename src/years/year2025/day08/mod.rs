use std::{cmp::Reverse, collections::HashSet, i128};

use crate::{
    solver::solver::Solver,
    utils::{
        file_reader::FileReader,
        math_utils::{Point3D, calculate_euclidean_distance_squared},
        memory_utils::create_memory_heap_from_vec,
    },
};

pub struct Day08 {
    largest_three_circuits: i128,
    last_connection_product: i128,
}

impl Solver<i128> for Day08 {
    fn new<R: FileReader>(reader: &R, file_path: &str) -> Result<Self, String> {
        let data = reader.read_file(file_path)?;
        let coordinates: Vec<Point3D> = data
            .split("\n")
            .map(|s| {
                let arr: Vec<i128> = s.split(",").map(|x| x.parse::<i128>().unwrap()).collect();
                (arr[0], arr[1], arr[2])
            })
            .collect();

        let mut heap = create_memory_heap_from_vec(&coordinates, |a, b, i, j| {
            let distance = calculate_euclidean_distance_squared(*a, *b);
            Reverse((distance, i, j))
        });

        let mut circuits: Vec<HashSet<Point3D>> = Vec::new();
        let mut pairs_processed = 0;
        while pairs_processed < 1000 {
            if let Some(Reverse((_, i, j))) = heap.pop() {
                pairs_processed += 1;

                let coords = (coordinates[i], coordinates[j]);

                // Skip the merge logic if already in same circuit
                if coordinate_in_same_circuit(&circuits, coords) {
                    continue;
                }

                // junction box logic:
                let first_junction_box = coordinate_junction_box(&circuits, coords.0);
                let second_junction_box = coordinate_junction_box(&circuits, coords.1);

                // if neither coordinate are in junction box, place that pair in a new junction box
                if first_junction_box.is_none() && second_junction_box.is_none() {
                    let mut new_circuit = HashSet::new();
                    new_circuit.insert(coords.0);
                    new_circuit.insert(coords.1);
                    circuits.push(new_circuit);
                }

                // check if at least one of those is in a junction box and connect it
                if first_junction_box.is_some() && second_junction_box.is_none() {
                    let idx = first_junction_box.unwrap();
                    circuits[idx].insert(coords.1);
                }

                if first_junction_box.is_none() && second_junction_box.is_some() {
                    let idx = second_junction_box.unwrap();
                    circuits[idx].insert(coords.0);
                }

                // in this case we need to merge them
                if first_junction_box.is_some() && second_junction_box.is_some() {
                    let idx1 = first_junction_box.unwrap();
                    let idx2 = second_junction_box.unwrap();

                    if idx1 != idx2 {
                        let (target_idx, remove_idx) = if idx1 < idx2 {
                            (idx1, idx2)
                        } else {
                            (idx2, idx1)
                        };
                        let circuit_to_merge = circuits.remove(remove_idx);
                        circuits[target_idx].extend(circuit_to_merge);
                    }
                }
            } else {
                break;
            }
        }

        circuits.sort_by(|a, b| b.len().cmp(&a.len()));
        let largest_three_circuits: usize = circuits.iter().take(3).map(|c| c.len()).product();

        // === Part 2: Continue until 1 circuit ===
        let mut last_merged_pair: Option<(Point3D, Point3D)> = None;
        while circuits.len() > 1
            || circuits.iter().map(|c| c.len()).sum::<usize>() < coordinates.len()
        {
            if let Some(Reverse((_, i, j))) = heap.pop() {
                let coords = (coordinates[i], coordinates[j]);

                if coordinate_in_same_circuit(&circuits, coords) {
                    continue;
                }

                last_merged_pair = Some(coords);

                // junction box logic:
                let first_junction_box = coordinate_junction_box(&circuits, coords.0);
                let second_junction_box = coordinate_junction_box(&circuits, coords.1);

                // if neither coordinate are in junction box, place that pair in a new junction box
                if first_junction_box.is_none() && second_junction_box.is_none() {
                    let mut new_circuit = HashSet::new();
                    new_circuit.insert(coords.0);
                    new_circuit.insert(coords.1);
                    circuits.push(new_circuit);
                }

                // check if at least one of those is in a junction box and connect it
                if first_junction_box.is_some() && second_junction_box.is_none() {
                    let idx = first_junction_box.unwrap();
                    circuits[idx].insert(coords.1);
                }

                if first_junction_box.is_none() && second_junction_box.is_some() {
                    let idx = second_junction_box.unwrap();
                    circuits[idx].insert(coords.0);
                }

                // in this case we need to merge them
                if first_junction_box.is_some() && second_junction_box.is_some() {
                    let idx1 = first_junction_box.unwrap();
                    let idx2 = second_junction_box.unwrap();

                    if idx1 != idx2 {
                        let (target_idx, remove_idx) = if idx1 < idx2 {
                            (idx1, idx2)
                        } else {
                            (idx2, idx1)
                        };
                        let circuit_to_merge = circuits.remove(remove_idx);
                        circuits[target_idx].extend(circuit_to_merge);
                    }
                }
            } else {
                break;
            }
        }

        let result = last_merged_pair.unwrap();
        let answer = result.0.0 * result.1.0;

        Ok(Day08 {
            largest_three_circuits: largest_three_circuits as i128,
            last_connection_product: answer as i128,
        })
    }

    fn part_one_solution(&mut self) -> i128 {
        self.largest_three_circuits
    }

    fn part_two_solution(&mut self) -> i128 {
        self.last_connection_product
    }
}

fn coordinate_in_same_circuit(circuits: &[HashSet<Point3D>], coords: (Point3D, Point3D)) -> bool {
    circuits
        .iter()
        .find(|c| c.contains(&coords.0) && c.contains(&coords.1))
        .is_some()
}

fn coordinate_junction_box(circuits: &[HashSet<Point3D>], coord: Point3D) -> Option<usize> {
    circuits.iter().position(|circuit| circuit.contains(&coord))
}
