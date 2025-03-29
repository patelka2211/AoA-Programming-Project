use data_types::{Point, Points};

pub fn merge_sort_points<F>(points: &Points, compare: F) -> Points
where
    F: Fn(&Point, &Point) -> bool + Copy,
{
    if points.len() <= 1 {
        return points.clone();
    }

    let mid = points.len() / 2;
    let left = merge_sort_points(&points[0..mid].to_vec(), compare);
    let right = merge_sort_points(&points[mid..].to_vec(), compare);

    merge(&left, &right, compare)
}

fn merge<F>(left: &Points, right: &Points, compare: F) -> Points
where
    F: Fn(&Point, &Point) -> bool + Copy,
{
    let mut result = Vec::new();
    let mut left_index = 0;
    let mut right_index = 0;

    while left_index < left.len() && right_index < right.len() {
        if compare(&left[left_index], &right[right_index]) {
            result.push(left[left_index].clone());
            left_index += 1;
        } else {
            result.push(right[right_index].clone());
            right_index += 1;
        }
    }

    result.extend_from_slice(&left[left_index..]);
    result.extend_from_slice(&right[right_index..]);

    result
}

pub fn sort_points_by_x(points: &Points) -> Points {
    merge_sort_points(points, |point1, point2| point1.x < point2.x)
}

pub fn sort_points_by_y(points: &Points) -> Points {
    merge_sort_points(points, |point1, point2| point1.y < point2.y)
}
