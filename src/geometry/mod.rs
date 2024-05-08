use num_integer::Roots;
use std::cmp::Ordering;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

mod point;
mod polygon;
mod triangle;

pub use point::Point;
pub use polygon::Polygon;
pub use triangle::Triangle;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn distance(a: Point, b: Point) -> u16 {
    let dx = (a.x as i32 - b.x as i32).unsigned_abs() as u16;
    let dy = (a.y as i32 - b.y as i32).unsigned_abs() as u16;
    (dx as u64 * dx as u64 + dy as u64 * dy as u64).sqrt() as u16
}

pub fn is_point_inside_triangle(triangle: &Triangle, point: Point) -> bool {
    let a = triangle.0;
    let b = triangle.1;
    let c = triangle.2;

    let cross_product = |a: Point, b: Point, c: Point| {
        ((b.x as i32 - a.x as i32) * (c.y as i32 - a.y as i32))
            - ((b.y as i32 - a.y as i32) * (c.x as i32 - a.x as i32))
    };

    matches!(
        (
            cross_product(a, b, point).cmp(&0),
            cross_product(b, c, point).cmp(&0),
            cross_product(c, a, point).cmp(&0),
        ),
        (Ordering::Less, Ordering::Less, Ordering::Less)
            | (Ordering::Greater, Ordering::Greater, Ordering::Greater)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 3, y: 4 };
        assert_eq!(distance(a, b), 5);
    }

    #[test]
    fn test_is_point_inside_triangle_on_side() {
        let triangle = Triangle(
            Point { x: 0, y: 0 },
            Point { x: 3, y: 4 },
            Point { x: 4, y: 0 },
        );
        let point = Point { x: 2, y: 2 };
        assert!(is_point_inside_triangle(&triangle, point));
    }

    #[test]
    fn test_is_point_inside_triangle() {
        let triangle = Triangle(
            Point { x: 0, y: 0 },
            Point { x: 3, y: 4 },
            Point { x: 4, y: 0 },
        );
        let point = Point { x: 2, y: 1 };
        assert!(is_point_inside_triangle(&triangle, point));
    }

    #[test]
    fn test_is_point_outside_triangle() {
        let triangle = Triangle(
            Point { x: 0, y: 0 },
            Point { x: 3, y: 4 },
            Point { x: 4, y: 0 },
        );
        let point = Point { x: 2, y: 3 };
        assert!(!is_point_inside_triangle(&triangle, point));
    }
}
