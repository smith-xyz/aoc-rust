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
            ((1, 1), (0, 0), 2),
            ((2, 2), (2, 5), 1),
            ((7, 1), (11, 7), 5),
        ];
        for case in test_cases {
            assert_eq!(x_axis_diff_inclusive(case.0, case.1), case.2)
        }
    }

    #[test]
    fn test_y_axis_diff() {
        let test_cases = [((1, 1), (0, 0), 2), ((2, 2), (2, 5), 4)];
        for case in test_cases {
            assert_eq!(y_axis_diff_inclusive(case.0, case.1), case.2)
        }
    }
}
