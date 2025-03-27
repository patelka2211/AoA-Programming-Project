mod recursive;
mod sort_by;
mod strip;

use std::time::Instant;

use data_types::{ClosestPairWithDuration, Points};
use recursive::closest_pair_recursive;
use sort_by::{sort_points_by_x, sort_points_by_y};

pub fn closest_pair(points: &Points) -> ClosestPairWithDuration {
    // Sort points by x-coordinate
    let px = sort_points_by_x(points);

    // Sort points by y-coordinate
    let py = sort_points_by_y(points);

    let start_time = Instant::now();

    let closest_pair = closest_pair_recursive(&px, &py);

    ClosestPairWithDuration {
        duration: start_time.elapsed(),
        closest_pair,
    }
}
