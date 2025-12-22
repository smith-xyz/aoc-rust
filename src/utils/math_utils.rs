// types

use std::cmp::{max, min};

pub type Point2D = (i128, i128);
pub type Point3D = (i128, i128, i128);

// functions

pub fn find_divisors(v: &str) -> Vec<usize> {
    let n = v.len();
    let mut divisors = Vec::new();
    let sqrt_n = (n as f64).sqrt() as usize;

    for i in 1..=sqrt_n {
        if n % i == 0 {
            divisors.push(i);
            let pair = n / i;
            if pair != i {
                divisors.push(pair);
            }
        }
    }
    divisors.sort(); // Optional: if you need sorted order
    divisors
}

// just so I don't have to use f64
pub fn calculate_euclidean_distance_squared(pos_1: Point3D, pos_2: Point3D) -> i128 {
    let dx = pos_1.0 - pos_2.0;
    let dy = pos_1.1 - pos_2.1;
    let dz = pos_1.2 - pos_2.2;
    dx * dx + dy * dy + dz * dz
}

pub fn calculate_area(length: i128, width: i128) -> i128 {
    length * width
}

pub fn lowest_2d_point(coord_1: Point2D, coord_2: Point2D) -> Point2D {
    if coord_1.0 < coord_2.0 && coord_1.1 <= coord_2.1 {
        coord_1
    } else {
        coord_2
    }
}

pub fn highest_2d_point(coord_1: Point2D, coord_2: Point2D) -> Point2D {
    if coord_1.0 > coord_2.0 && coord_1.1 >= coord_2.1 {
        coord_1
    } else {
        coord_2
    }
}

pub fn highest_y_point(coord_1: Point2D, coord_2: Point2D) -> Point2D {
    if coord_1.1 > coord_2.1 {
        coord_1
    } else if coord_2.1 > coord_1.1 {
        coord_2
    } else {
        if coord_1.0 > coord_2.0 {
            coord_1
        } else {
            coord_2
        }
    }
}

pub fn lowest_y_point(coord_1: Point2D, coord_2: Point2D) -> Point2D {
    if coord_1.1 < coord_2.1 {
        coord_1
    } else if coord_2.1 < coord_1.1 {
        coord_2
    } else {
        if coord_1.0 < coord_2.0 {
            coord_1
        } else {
            coord_2
        }
    }
}

pub fn x_axis_diff_inclusive(coord_1: Point2D, coord_2: Point2D) -> i128 {
    let min_x = min(coord_1.0, coord_2.0);
    let max_x = max(coord_1.0, coord_2.0);
    (min_x - max_x) - 1
}

pub fn y_axis_diff_inclusive(coord_1: Point2D, coord_2: Point2D) -> i128 {
    let min_y = min(coord_1.1, coord_2.1);
    let max_y: i128 = max(coord_1.1, coord_2.1);
    (min_y - max_y) - 1
}

pub fn get_rectangle_perimeter_points(coord_1: Point2D, coord_2: Point2D) -> Vec<Point2D> {
    let min_x = min(coord_1.0, coord_2.0);
    let max_x = max(coord_1.0, coord_2.0);
    let min_y = min(coord_1.1, coord_2.1);
    let max_y: i128 = max(coord_1.1, coord_2.1);

    let mut coordinates = Vec::new();
    coordinates.extend((min_x..=max_x).map(|x| (x, min_y)));
    coordinates.extend((min_x..=max_x).map(|x| (x, max_y)));
    coordinates.extend(((min_y + 1)..max_y).map(|y| (min_x, y)));
    coordinates.extend(((min_y + 1)..max_y).map(|y| (max_x, y)));
    coordinates
}

pub fn get_rectangle_perimeter_lines(coord_1: Point2D, coord_2: Point2D) -> Vec<Vec<Point2D>> {
    let min_x = min(coord_1.0, coord_2.0);
    let max_x = max(coord_1.0, coord_2.0);
    let min_y = min(coord_1.1, coord_2.1);
    let max_y: i128 = max(coord_1.1, coord_2.1);

    let mut coordinates = Vec::new();
    coordinates.push((min_x..=max_x).map(|x| (x, min_y)).collect());
    coordinates.push((min_x..=max_x).map(|x| (x, max_y)).collect());
    coordinates.push((min_y..=max_y).map(|y| (min_x, y)).collect());
    coordinates.push((min_y..=max_y).map(|y| (max_x, y)).collect());

    coordinates
}

pub fn is_point_in_range(coord_1: Point2D, coord_2: Point2D, p: Point2D) -> bool {
    let min_x = min(coord_1.0, coord_2.0);
    let max_x = max(coord_1.0, coord_2.0);
    let min_y = min(coord_1.1, coord_2.1);
    let max_y: i128 = max(coord_1.1, coord_2.1);
    min_x <= p.0 && p.0 <= max_x && min_y <= p.1 && p.1 <= max_y
}

pub fn is_point_in_polygon(point: Point2D, polygon: &[Point2D]) -> bool {
    if polygon.len() < 3 {
        return false;
    }

    let mut inside = false;
    let n = polygon.len();

    for i in 0..n {
        let j = (i + 1) % n;
        let vi = polygon[i];
        let vj = polygon[j];

        let intersect = ((vi.1 > point.1) != (vj.1 > point.1))
            && (point.0 < (vj.0 - vi.0) * (point.1 - vi.1) / (vj.1 - vi.1) + vi.0);

        if intersect {
            inside = !inside;
        }
    }

    inside
}

pub fn rotate_grid_points(points: &[(usize, usize)], degrees: i32) -> Vec<(usize, usize)> {
    let signed_points: Vec<(i32, i32)> =
        points.iter().map(|&(r, c)| (r as i32, c as i32)).collect();

    let rotated: Vec<(i32, i32)> = signed_points
        .iter()
        .map(|&(x, y)| match degrees {
            90 => (y, -x),
            180 => (-x, -y),
            270 => (-y, x),
            _ => (x, y), // 0 or 360
        })
        .collect();

    // shift to origin
    normalize_and_convert(rotated)
}

pub fn reflect_grid_points_horizontal(points: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let signed_points: Vec<(i32, i32)> =
        points.iter().map(|&(r, c)| (r as i32, c as i32)).collect();

    // (x, y) -> (x, -y)
    let flipped: Vec<(i32, i32)> = signed_points.iter().map(|&(x, y)| (x, -y)).collect();

    normalize_and_convert(flipped)
}

pub fn reflect_grid_points_vertical(points: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let signed_points: Vec<(i32, i32)> =
        points.iter().map(|&(r, c)| (r as i32, c as i32)).collect();

    // (x, y) -> (-x, y)
    let flipped: Vec<(i32, i32)> = signed_points.iter().map(|&(x, y)| (-x, y)).collect();

    normalize_and_convert(flipped)
}

// Helper: normalize coordinates to start at (0,0) and convert to usize
fn normalize_and_convert(points: Vec<(i32, i32)>) -> Vec<(usize, usize)> {
    if points.is_empty() {
        return Vec::new();
    }

    let min_x = points.iter().map(|&(x, _)| x).min().unwrap();
    let min_y = points.iter().map(|&(_, y)| y).min().unwrap();

    points
        .iter()
        .map(|&(x, y)| ((x - min_x) as usize, (y - min_y) as usize))
        .collect()
}

pub fn sort_coords(points: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut sorted = points.to_vec();
    sorted.sort();
    sorted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_divisors() {
        let mut res: Vec<usize> = find_divisors("123123123");
        assert_eq!(res, vec![1, 3, 9]);

        res = find_divisors("22");
        assert_eq!(res, vec![1, 2]);

        res = find_divisors("38593859");
        assert_eq!(res, vec![1, 2, 4, 8])
    }

    #[test]
    fn test_lowest_2d_point() {
        let test_cases = [((1, 1), (0, 0), (0, 0)), ((2, 1), (7, 1), (2, 1))];
        for case in test_cases {
            assert_eq!(lowest_2d_point(case.0, case.1), case.2)
        }
    }

    #[test]
    fn test_x_axis_diff() {
        let test_cases = [
            ((1, 1), (0, 0), -2),
            ((2, 2), (2, 5), -1),
            ((7, 1), (11, 7), -5),
        ];
        for case in test_cases {
            assert_eq!(x_axis_diff_inclusive(case.0, case.1), case.2)
        }
    }

    #[test]
    fn test_y_axis_diff() {
        let test_cases = [((1, 1), (0, 0), -2), ((2, 2), (2, 5), -4)];
        for case in test_cases {
            assert_eq!(y_axis_diff_inclusive(case.0, case.1), case.2)
        }
    }

    #[test]
    fn test_rotate_grid_points() {
        // Simple L-shape: ##
        //                  #.
        let shape = vec![(0, 0), (0, 1), (1, 0)];
        let mut result = rotate_grid_points(&shape, 0);
        result.sort();
        let mut expected = vec![(0, 0), (0, 1), (1, 0)];
        expected.sort();
        assert_eq!(result, expected, "0° rotation should be identity");

        // 90° clockwise: .#
        //                 ##
        result = rotate_grid_points(&shape, 90);
        result.sort();
        expected = vec![(0, 0), (0, 1), (1, 1)];
        expected.sort();
        assert_eq!(result, expected, "90° rotation");

        // 180° clockwise: .#
        //                  ##
        result = rotate_grid_points(&shape, 180);
        result.sort();
        expected = vec![(0, 1), (1, 0), (1, 1)];
        expected.sort();
        assert_eq!(result, expected, "180° rotation");

        // 270° clockwise: #.
        //                  ##
        result = rotate_grid_points(&shape, 270);
        result.sort();
        expected = vec![(0, 0), (1, 0), (1, 1)];
        expected.sort();
        assert_eq!(result, expected, "270° rotation");

        // Test that 360° is same as 0°
        result = rotate_grid_points(&shape, 360);
        result.sort();
        expected = vec![(0, 0), (0, 1), (1, 0)];
        expected.sort();
        assert_eq!(result, expected, "360° rotation should be identity");
    }

    #[test]
    fn test_rotate_grid_points_normalization() {
        // Shape starting at (2, 3) should normalize to (0, 0)
        let shape = vec![(2, 3), (2, 4), (3, 3)];
        let result = rotate_grid_points(&shape, 90);
        // All coordinates should be normalized (start at 0,0)
        let min_row = result.iter().map(|&(r, _)| r).min().unwrap();
        let min_col = result.iter().map(|&(_, c)| c).min().unwrap();
        assert_eq!(
            min_row, 0,
            "Rotated shape should be normalized (min row = 0)"
        );
        assert_eq!(
            min_col, 0,
            "Rotated shape should be normalized (min col = 0)"
        );
    }

    #[test]
    fn test_reflect_grid_points_horizontal() {
        // Original: ##
        //           #.
        let shape = vec![(0, 0), (0, 1), (1, 0)];
        let mut result = reflect_grid_points_horizontal(&shape);
        result.sort();
        // Horizontal flip: #.
        //                  ##
        let mut expected = vec![(0, 0), (0, 1), (1, 1)];
        expected.sort();
        assert_eq!(result, expected, "Horizontal flip");

        // Test with offset shape
        let offset_shape = vec![(2, 1), (2, 2), (3, 1)];
        result = reflect_grid_points_horizontal(&offset_shape);
        let min_row = result.iter().map(|&(r, _)| r).min().unwrap();
        let min_col = result.iter().map(|&(_, c)| c).min().unwrap();
        assert_eq!(min_row, 0, "Flipped shape should be normalized");
        assert_eq!(min_col, 0, "Flipped shape should be normalized");
    }

    #[test]
    fn test_reflect_grid_points_vertical() {
        // Original: ##
        //           #.
        let shape = vec![(0, 0), (0, 1), (1, 0)];
        let mut result = reflect_grid_points_vertical(&shape);
        result.sort();
        // Vertical flip: ##
        //                .#
        let mut expected = vec![(0, 0), (1, 0), (1, 1)];
        expected.sort();
        assert_eq!(result, expected, "Vertical flip");
    }

    #[test]
    fn test_rotation_and_reflection_consistency() {
        // Test that rotating 4 times returns to original
        let shape = vec![(0, 0), (0, 1), (1, 0), (1, 1), (2, 0)];
        let rotated_90 = rotate_grid_points(&shape, 90);
        let rotated_180 = rotate_grid_points(&rotated_90, 90);
        let rotated_270 = rotate_grid_points(&rotated_180, 90);
        let rotated_360 = rotate_grid_points(&rotated_270, 90);

        let mut original_sorted = shape.clone();
        original_sorted.sort();
        let mut final_sorted = rotated_360;
        final_sorted.sort();

        assert_eq!(
            original_sorted, final_sorted,
            "4 rotations of 90° should return to original"
        );
    }

    #[test]
    fn test_empty_shape() {
        let empty: Vec<(usize, usize)> = vec![];
        assert_eq!(rotate_grid_points(&empty, 90), empty);
        assert_eq!(reflect_grid_points_horizontal(&empty), empty);
        assert_eq!(reflect_grid_points_vertical(&empty), empty);
    }

    #[test]
    fn test_single_point() {
        let single = vec![(5, 7)];
        let result = rotate_grid_points(&single, 90);
        assert_eq!(
            result,
            vec![(0, 0)],
            "Single point should normalize to (0,0)"
        );

        let result_h = reflect_grid_points_horizontal(&single);
        assert_eq!(result_h, vec![(0, 0)]);

        let result_v = reflect_grid_points_vertical(&single);
        assert_eq!(result_v, vec![(0, 0)]);
    }
}
