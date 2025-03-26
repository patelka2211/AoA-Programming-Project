use std::cmp::min;

use data_types::{AlgorithmOutput, Point};

fn sort_points_by_x(points: &Vec<Point>) -> Vec<Point> {
    let mut sorted_points = points.clone();

    sorted_points.sort_by(|point1, point2| point1.x.partial_cmp(&point2.x).unwrap());

    sorted_points
}

fn sort_points_by_y(points: &Vec<Point>) -> Vec<Point> {
    let mut sorted_points = points.clone();

    sorted_points.sort_by(|point1, point2| point1.y.partial_cmp(&point2.y).unwrap());

    sorted_points
}

fn closest_pair_in_strip(points_inside_strip: &Vec<Point>) -> (f64, Option<(Point, Point)>) {
    let mut min_distance = f64::MAX;

    let mut closest_pair = None;

    // Traverse from 0th to (n - 1)th element in strip array.
    for outer_index in 0..points_inside_strip.len() {
        let point0 = points_inside_strip.get(outer_index).unwrap();

        // Compute distance from point0 to next 15 points in the strip.
        for inner_index in (outer_index + 1)..min(outer_index + 16, points_inside_strip.len()) {
            let point1 = points_inside_strip.get(inner_index).unwrap();

            let distance = point0.distance(point1);

            if distance < min_distance {
                min_distance = distance;
                closest_pair = Some((point0, point1));
            }
        }
    }

    if let Some((point0, point1)) = closest_pair {
        (min_distance, Some((point0.clone(), point1.clone())))
    } else {
        (min_distance, None)
    }
}

fn closest_pair_recursive(px: &Vec<Point>, py: &Vec<Point>) -> Option<(Point, Point)> {
    let n = px.len();

    // BASE CASE
    // If there are less than 3 points then simply find the closest pair and return.
    if n <= 3 {
        let (_, closest_pair_strip) = closest_pair_in_strip(py);
        return closest_pair_strip;
    }

    // Divide the points into two halves
    let mid = n / 2;

    // Left half of the points sorted by x-coordinate
    let qx = px[0..mid].to_vec();
    // Right half of the points sorted by x-coordinate
    let rx = px[mid..n].to_vec();

    // Left half of the points sorted by y-coordinate
    let qy = py[0..mid].to_vec();
    // Right half of the points sorted by y-coordinate
    let ry = py[mid..n].to_vec();

    // Recursively find the closest pair in each half
    let (q0, q1) = closest_pair_recursive(&qx, &qy).unwrap();
    let (r0, r1) = closest_pair_recursive(&rx, &ry).unwrap();

    // Determine the minimum distance from the two halves
    let delta_left = q0.distance(&q1);
    let delta_right = r0.distance(&r1);

    let (delta_minimum, mut closest_pair) = if delta_left < delta_right {
        (delta_left, Some((q0, q1)))
    } else {
        (delta_right, Some((r0, r1)))
    };

    let l = px[mid].x;

    // Create a strip of points closer than minimum delta to the midpoint
    let mut strip = Vec::new();
    for p in px {
        if ((p.x as i128 - l as i128).abs() as f64) < delta_minimum {
            strip.push(p.clone());
        }
    }

    // Sort the strip by y-coordinate
    let strip = sort_points_by_y(&strip);

    let (min_distance_strip, closest_pair_strip) = closest_pair_in_strip(&strip);

    if min_distance_strip < delta_minimum {
        closest_pair = closest_pair_strip;
    }

    closest_pair
}

pub fn closest_pair(points: &Vec<Point>) -> AlgorithmOutput {
    // Sort points by x-coordinate
    let px = sort_points_by_x(points);

    // Sort points by y-coordinate
    let py = sort_points_by_y(points);

    // Start timing
    let start_time = std::time::Instant::now();

    let closest_pair = closest_pair_recursive(&px, &py);

    // Start timing
    let stop_time = std::time::Instant::now();

    AlgorithmOutput {
        closest_pair,
        time_elapsed: (stop_time - start_time).as_millis(),
    }
}
