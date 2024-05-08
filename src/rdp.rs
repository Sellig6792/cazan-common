use crate::geometry::{distance, Point, Polygon};

pub fn rdp(polygon: &Polygon, epsilon: f64) -> Polygon {
    let mut d_max = 0.0;
    let mut index = 0;
    let end = polygon.len() - 1;

    for i in 1..end {
        let d = perpendicular_distance(polygon[i], polygon[0], polygon[end]);
        if d > d_max {
            index = i;
            d_max = d;
        }
    }

    if d_max > epsilon {
        let mut results = rdp(&Vec::from(&polygon[..=index]), epsilon);
        results.pop();
        results.extend(rdp(&Vec::from(&polygon[index..]), epsilon));
        results
    } else {
        vec![polygon[0], polygon[end]]
    }
}

fn perpendicular_distance(p: Point, a: Point, b: Point) -> f64 {
    let num = ((b.y as i32 - a.y as i32) * p.x as i32) - ((b.x as i32 - a.x as i32) * p.y as i32)
        + (b.x as i32 * a.y as i32)
        - (b.y as i32 * a.x as i32);
    let num = num.abs() as f64;
    let den = distance(a, b) as f64;
    num / den
}
