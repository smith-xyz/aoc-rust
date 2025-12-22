use std::collections::{HashMap, HashSet};

use rayon::prelude::*;

use crate::{
    solver::solver::Solver,
    utils::{
        file_reader::FileReader,
        math_utils::{
            reflect_grid_points_horizontal, reflect_grid_points_vertical, rotate_grid_points,
            sort_coords,
        },
    },
};

type Shapes = HashMap<usize, Vec<(usize, usize)>>;
type Regions = Vec<((usize, usize), Vec<usize>)>;

pub struct Day12 {
    shapes: Shapes,
    regions: Regions,
}

fn generate_all_shape_variants(points: &[(usize, usize)]) -> Vec<Vec<(usize, usize)>> {
    let mut variants = Vec::new();
    let mut seen = HashSet::new();

    for degrees in [0, 90, 180, 270] {
        let rotated = rotate_grid_points(points, degrees);
        let rotated_sorted = sort_coords(&rotated);

        if seen.insert(rotated_sorted.clone()) {
            variants.push(rotated.clone());
        }

        let flipped_h = reflect_grid_points_horizontal(&rotated);
        let flipped_h_sorted = sort_coords(&flipped_h);
        if seen.insert(flipped_h_sorted) {
            variants.push(flipped_h);
        }

        let flipped_v = reflect_grid_points_vertical(&rotated);
        let flipped_v_sorted = sort_coords(&flipped_v);
        if seen.insert(flipped_v_sorted) {
            variants.push(flipped_v);
        }
    }

    variants
}

type Placement = Vec<usize>;

fn precompute_placements(
    width: usize,
    height: usize,
    variants: &[Vec<(usize, usize)>],
) -> Vec<Placement> {
    let mut placements = Vec::new();

    for variant in variants {
        let max_row = variant.iter().map(|&(r, _)| r).max().unwrap_or(0);
        let max_col = variant.iter().map(|&(_, c)| c).max().unwrap_or(0);

        for start_row in 0..=height.saturating_sub(max_row + 1) {
            for start_col in 0..=width.saturating_sub(max_col + 1) {
                let cells: Placement = variant
                    .iter()
                    .map(|&(r, c)| (start_row + r) * width + (start_col + c))
                    .collect();
                placements.push(cells);
            }
        }
    }

    placements
}

fn can_pack_shapes(width: usize, height: usize, all_variants: &[Vec<Vec<(usize, usize)>>]) -> bool {
    let mut all_placements: Vec<(usize, Vec<Placement>)> = all_variants
        .iter()
        .enumerate()
        .map(|(idx, variants)| (idx, precompute_placements(width, height, variants)))
        .collect();

    all_placements.sort_by_key(|(_, placements)| placements.len());

    let sorted_placements: Vec<Vec<Placement>> =
        all_placements.into_iter().map(|(_, p)| p).collect();

    let grid_size = width * height;
    let mut occupied = vec![false; grid_size];

    backtrack_pack(&mut occupied, &sorted_placements, 0)
}

fn backtrack_pack(
    occupied: &mut Vec<bool>,
    all_placements: &[Vec<Placement>],
    shape_idx: usize,
) -> bool {
    if shape_idx >= all_placements.len() {
        return true;
    }

    for placement in &all_placements[shape_idx] {
        if placement.iter().all(|&cell| !occupied[cell]) {
            for &cell in placement {
                occupied[cell] = true;
            }

            if backtrack_pack(occupied, all_placements, shape_idx + 1) {
                return true;
            }

            for &cell in placement {
                occupied[cell] = false;
            }
        }
    }

    false
}

impl Solver<u32> for Day12 {
    fn new<R: FileReader>(reader: &R, file_path: &str) -> Result<Self, String> {
        let data = reader.read_file(file_path)?;
        let shape_blocks: Vec<&str> = data.split("\n\n").collect();

        let mut shapes: Shapes = HashMap::new();
        let mut regions: Regions = Vec::new();

        for block in shape_blocks {
            if block.contains(":") && !block.contains("x") {
                let mut lines = block.lines();
                let header = lines.next().unwrap();
                let shape_id: usize = header.trim_end_matches(":").parse::<usize>().unwrap();

                let mut coords = Vec::new();
                for (row, line) in lines.enumerate() {
                    for (col, ch) in line.chars().enumerate() {
                        if ch == '#' {
                            coords.push((row, col));
                        }
                    }
                }

                if !coords.is_empty() {
                    let min_row = coords.iter().map(|&(r, _)| r).min().unwrap();
                    let min_col = coords.iter().map(|&(_, c)| c).min().unwrap();
                    let normalized: Vec<(usize, usize)> = coords
                        .iter()
                        .map(|&(r, c)| (r - min_row, c - min_col))
                        .collect();
                    shapes.entry(shape_id).insert_entry(normalized);
                }
            } else {
                for line in block.lines() {
                    if let Some((dims, shape_counts_str)) = line.split_once(": ") {
                        let (width, height) = dims
                            .split_once('x')
                            .ok_or_else(|| format!("Invalid dimensions: {}", dims))?;
                        let width: usize = width.parse::<usize>().unwrap();
                        let height: usize = height.parse::<usize>().unwrap();

                        let shape_counts: Vec<usize> = shape_counts_str
                            .split_whitespace()
                            .map(|s| s.parse::<usize>().unwrap())
                            .collect();

                        let mut shape_list = Vec::new();
                        for (shape_id, &count) in shape_counts.iter().enumerate() {
                            for _ in 0..count {
                                shape_list.push(shape_id);
                            }
                        }

                        regions.push(((width, height), shape_list));
                    }
                }
            }
        }

        Ok(Day12 { shapes, regions })
    }

    fn part_one_solution(&mut self) -> u32 {
        self.regions
            .par_iter()
            .filter(|((grid_w, grid_h), shape_indices)| {
                let shape_list: Vec<Vec<(usize, usize)>> = shape_indices
                    .iter()
                    .filter_map(|&id| self.shapes.get(&id).cloned())
                    .collect();

                if shape_list.len() != shape_indices.len() {
                    return false;
                }

                let total_area: usize = shape_list.iter().map(|s| s.len()).sum();
                let grid_area = grid_w * grid_h;

                if total_area > grid_area {
                    return false;
                }

                let all_variants: Vec<Vec<Vec<(usize, usize)>>> = shape_list
                    .iter()
                    .map(|s| generate_all_shape_variants(s))
                    .collect();

                can_pack_shapes(*grid_w, *grid_h, &all_variants)
            })
            .count() as u32
    }

    fn part_two_solution(&mut self) -> u32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::file_reader::StdFileReader;

    use super::*;

    #[test]
    fn test_part_one() {
        let reader = StdFileReader;
        let mut solver = Day12::from_test_path(&reader, 2025, 12).expect("Failed to load input");
        assert_eq!(solver.part_one_solution(), 2)
    }

    #[test]
    fn test_part_two() {
        let reader = StdFileReader;
        let mut solver = Day12::from_test_path(&reader, 2025, 12).expect("Failed to load input");
        assert_eq!(solver.part_two_solution(), 0)
    }
}
