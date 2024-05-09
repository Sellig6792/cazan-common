use crate::geometry::*;
use earcutr::earcut;

pub fn triangulate(
    polygon: &Polygon,
    holes: Option<&Vec<Polygon>>,
) -> Result<Vec<Triangle>, earcutr::Error> {
    let mut points: Vec<f64> = polygon
        .iter()
        .flat_map(|point| vec![point.x as f64, point.y as f64])
        .collect();
    let mut holes_indices: Vec<usize> = vec![]; // indices of the first point of each hole in the points array

    let mut polygon_with_holes = polygon.clone();
    // Remove the holes that only have 1 or 2 points

    if let Some(holes) = holes {
        let holes: Vec<&Polygon> = holes.iter().filter(|hole| hole.len() > 2).collect();
        for hole in holes {
            points.extend(
                hole.iter()
                    .flat_map(|point| vec![point.x as f64, point.y as f64]),
            );
            holes_indices.push(points.len() / 2 - hole.len());
            polygon_with_holes.extend(hole);
        }
    }

    let indices = earcut(&points, &holes_indices, 2)?;

    Ok(indices
        .chunks(3)
        .map(|chunk| {
            Triangle(
                polygon_with_holes[chunk[0]],
                polygon_with_holes[chunk[1]],
                polygon_with_holes[chunk[2]],
            )
        })
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

        let triangles = match triangulate(&polygon, None) {
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

        let triangles = triangulate(&polygon, None).unwrap();

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

        let triangles = triangulate(&polygon, None).unwrap();

        assert_eq!(triangles.len(), polygon.len() - 2);
    }

    #[test]
    fn test_triangulate_triangle() {
        let polygon = vec![
            Point { x: 0, y: 0 },
            Point { x: 3, y: 4 },
            Point { x: 4, y: 0 },
        ];
        let triangles = match triangulate(&polygon, None) {
            Ok(triangles) => triangles,
            Err(_) => panic!("Error"),
        };

        assert_eq!(triangles.len(), 1);
        assert_eq!(triangles[0], Triangle(polygon[0], polygon[1], polygon[2]));
    }
}
