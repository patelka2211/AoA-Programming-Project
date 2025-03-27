mod recursive;
mod sort_by;
mod strip;

use data_types::{PairOfPoints, Points};
use recursive::closest_pair_recursive;
use sort_by::{sort_points_by_x, sort_points_by_y};

pub fn closest_pair(points: &Points) -> Option<PairOfPoints> {
    // Sort points by x-coordinate
    let px = sort_points_by_x(points);

    // Sort points by y-coordinate
    let py = sort_points_by_y(points);

    let closest_pair = closest_pair_recursive(&px, &py);

    closest_pair
}
