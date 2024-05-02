use crate::geometry::*;
use earcutr::earcut;


pub fn triangulate(polygon: &Polygon) -> Result<Vec<Triangle>, earcutr::Error> {
    if polygon.len() < 3 {
        return Ok(vec![Triangle(polygon[0], polygon[1], polygon[2])]);
    }

    let points: Vec<f64> = polygon
        .iter()
        .flat_map(|point| vec![point.x as f64, point.y as f64])
        .collect();
    let indices = earcut(&points, &[], 2)?;

    Ok(indices
        .chunks(3)
        .map(|chunk| Triangle(polygon[chunk[0]], polygon[chunk[1]], polygon[chunk[2]]))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangulate() {
        let polygon = vec![
            Point { x: 0, y: 0 },
            Point { x: 3, y: 4 },
            Point { x: 4, y: 0 },
            Point { x: 2, y: 2 },
        ];

        let triangles = match triangulate(&polygon) {
            Ok(triangles) => triangles,
            Err(_) => panic!("Error"),
        };

        assert_eq!(triangles.len(), polygon.len() - 2);
    }

    #[test]
    fn test_triangulate_2() {
        let polygon = vec![
            Point { x: 0, y: 0 },
            Point { x: 3, y: 4 },
            Point { x: 4, y: 0 },
            Point { x: 6, y: 1 },
            Point { x: 7, y: 0 },
            Point { x: 7, y: 3 },
            Point { x: 6, y: 5 },
            Point { x: 1, y: 8 },
        ];

        let triangles = triangulate(&polygon).unwrap();

        assert_eq!(triangles.len(), polygon.len() - 2);
    }

    #[test]
    fn test_triangulate_3() {
        let polygon = vec![
            Point { x: 0, y: 0 },
            Point { x: 4, y: 0 },
            Point { x: 5, y: 2 },
            Point { x: 8, y: 1 },
            Point { x: 10, y: 6 },
            Point { x: 6, y: 4 },
            Point { x: 10, y: 8 },
            Point { x: 6, y: 10 },
            Point { x: 6, y: 16 },
            Point { x: 5, y: 14 },
            Point { x: 4, y: 15 },
            Point { x: 0, y: 16 },
            Point { x: 2, y: 8 },
        ];

        let triangles = triangulate(&polygon).unwrap();

        assert_eq!(triangles.len(), polygon.len() - 2);
    }

    #[test]
    fn test_triangulate_triangle() {
        let polygon = vec![
            Point { x: 0, y: 0 },
            Point { x: 3, y: 4 },
            Point { x: 4, y: 0 },
        ];
        let triangles = match triangulate(&polygon) {
            Ok(triangles) => triangles,
            Err(_) => panic!("Error"),
        };

        assert_eq!(triangles.len(), 1);
        assert_eq!(triangles[0], Triangle(polygon[0], polygon[1], polygon[2]));
    }
}
