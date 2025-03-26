use data_types::Points;

pub fn sort_points_by_x(points: &Points) -> Points {
    let mut sorted_points = points.clone();

    sorted_points.sort_by(|point1, point2| point1.x.partial_cmp(&point2.x).unwrap());

    sorted_points
}

pub fn sort_points_by_y(points: &Points) -> Points {
    let mut sorted_points = points.clone();

    sorted_points.sort_by(|point1, point2| point1.y.partial_cmp(&point2.y).unwrap());

    sorted_points
}
