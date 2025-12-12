use std::collections::HashSet;

use crate::{
    solver::solver::Solver,
    utils::{
        file_reader::FileReader,
        math_utils::{
            Point2D, calculate_area, get_rectangle_perimeter_lines, x_axis_diff_inclusive,
            y_axis_diff_inclusive,
        },
    },
};
use itertools::Itertools;
use rayon::prelude::*;

pub struct Day09 {
    max_area: i128,
    max_area_within_green_tiles: i128,
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
}

type Tile = (Point2D, Color);

impl Solver<i128> for Day09 {
    fn new<R: FileReader>(reader: &R, file_path: &str) -> Result<Self, String> {
        let data = reader.read_file(file_path)?;
        // 2d grid
        let mut tiles: Vec<Tile> = data
            .split("\n")
            .map(|x| {
                let slice: Vec<i128> = x.split(",").map(|y| y.parse::<i128>().unwrap()).collect();
                ((slice[0], slice[1]), Color::Red)
            })
            .collect();

        let red_points: Vec<Point2D> = tiles.iter().map(|t| t.0).collect();

        // Generate green boundary tiles
        let mut green_tiles: Vec<Tile> = Vec::new();
        for i in 0..red_points.len() {
            let a = red_points[i];
            let b = red_points[(i + 1) % red_points.len()];

            let min_x = a.0.min(b.0);
            let max_x = a.0.max(b.0);
            let min_y = a.1.min(b.1);
            let max_y = a.1.max(b.1);

            if a.1 == b.1 {
                for x in (min_x + 1)..max_x {
                    green_tiles.push(((x, a.1), Color::Green));
                }
            } else if a.0 == b.0 {
                for y in (min_y + 1)..max_y {
                    green_tiles.push(((a.0, y), Color::Green));
                }
            }
        }
        tiles.extend(green_tiles);

        // Build vertical edges for ray casting
        let mut vertical_edges: Vec<(i128, i128, i128)> = Vec::new(); // (x, y_min, y_max)
        for i in 0..red_points.len() {
            let a = red_points[i];
            let b = red_points[(i + 1) % red_points.len()];
            if a.0 == b.0 {
                let y_min = a.1.min(b.1);
                let y_max = a.1.max(b.1);
                vertical_edges.push((a.0, y_min, y_max));
            }
        }

        // calculate all red-red pairs with areas, sorted by area descending
        let mut red_pairs: Vec<_> = red_points
            .iter()
            .tuple_combinations()
            .map(|(a, b)| {
                let area =
                    calculate_area(x_axis_diff_inclusive(*a, *b), y_axis_diff_inclusive(*a, *b))
                        .abs();
                (*a, *b, area)
            })
            .collect();

        red_pairs.sort_by(|a, b| b.2.cmp(&a.2));

        let max_area = red_pairs.first().map(|(_, _, area)| *area).unwrap_or(0);

        let boundary_points: HashSet<Point2D> = tiles.iter().map(|t| t.0).collect();
        // ray casting
        let is_inside_polygon = |point: Point2D| -> bool {
            if boundary_points.contains(&point) {
                return true; // On boundary = inside
            }
            let crossings = vertical_edges
                .iter()
                .filter(|&&(x, y_min, y_max)| x > point.0 && point.1 >= y_min && point.1 < y_max)
                .count();
            crossings % 2 == 1
        };

        let check_valid_idx = |idx: usize| -> bool {
            let (a, b, _) = &red_pairs[idx];
            let edges = get_rectangle_perimeter_lines(*a, *b);
            // Check every perimeter point - if any is outside, rectangle is invalid
            edges.iter().flatten().all(|p| is_inside_polygon(*p))
        };

        // splitting into chunks to optimize searching
        let num_chunks = 16;
        let chunk_size = (red_pairs.len() / num_chunks).max(1);

        let chunk_results: Vec<Option<usize>> = (0..num_chunks)
            .into_par_iter()
            .map(|chunk_idx| {
                let start = chunk_idx * chunk_size;
                let end = if chunk_idx == num_chunks - 1 {
                    red_pairs.len()
                } else {
                    (chunk_idx + 1) * chunk_size
                };

                // first valid
                for idx in start..end {
                    if check_valid_idx(idx) {
                        return Some(idx);
                    }
                }
                None
            })
            .collect();

        let first_valid_idx = chunk_results.into_iter().flatten().min();
        let max_area_within_green_tiles = first_valid_idx.map(|idx| red_pairs[idx].2).unwrap_or(0);

        Ok(Day09 {
            max_area,
            max_area_within_green_tiles,
        })
    }

    fn part_one_solution(&mut self) -> i128 {
        self.max_area
    }

    fn part_two_solution(&mut self) -> i128 {
        self.max_area_within_green_tiles
    }
}

// fn print_tiles(tiles: &[Tile], padding: i128) {
//     use std::collections::HashMap;

//     if tiles.is_empty() {
//         println!("No tiles to display");
//         return;
//     }

//     let raw_min_x = tiles.iter().map(|t| t.0.0).min().unwrap();
//     let raw_min_y = tiles.iter().map(|t| t.0.1).min().unwrap();

//     // Don't go below 0
//     let min_x = (raw_min_x - padding).max(0);
//     let max_x = tiles.iter().map(|t| t.0.0).max().unwrap() + padding;
//     let min_y = (raw_min_y - padding).max(0);
//     let max_y = tiles.iter().map(|t| t.0.1).max().unwrap() + padding;

//     println!(
//         "Bounds: x={}..{}, y={}..{}, tile count={}",
//         min_x,
//         max_x,
//         min_y,
//         max_y,
//         tiles.len()
//     );

//     let tile_map: HashMap<Point2D, &Color> = tiles.iter().map(|t| (t.0, &t.1)).collect();

//     for y in min_y..=max_y {
//         for x in min_x..=max_x {
//             let point = (x, y);
//             let ch = match tile_map.get(&point) {
//                 Some(Color::Red) => '#',
//                 Some(Color::Green) => 'X',
//                 None => '.',
//             };
//             print!("{}", ch);
//         }
//         println!(); // Add marker at end
//     }
//     println!();
// }
