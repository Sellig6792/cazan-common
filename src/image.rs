use crate::geometry::*;
use image;
use image::GenericImageView;

pub type Grid<T> = Vec<Vec<T>>;

pub struct ImagePolygon {
    image: image::DynamicImage,
    size: (u16, u16),
}

impl ImagePolygon {
    pub fn new(image: image::DynamicImage) -> Self {
        let dimensions = image.dimensions();
        let size = (dimensions.0 as u16, dimensions.1 as u16);
        Self { image, size }
    }

    fn get_opaque_points_grid(&self) -> Grid<bool> {
        let rgba = &self.image.to_rgba16();
        let mut opaque_points: Grid<bool> =
            vec![vec![false; self.size.0 as usize]; self.size.1 as usize];
        for y in 0..rgba.height() {
            for x in 0..rgba.width() {
                let pixel = rgba.get_pixel(x, y);
                let alpha_channel = pixel.0[3];
                opaque_points[y as usize][x as usize] = alpha_channel > 0;
            }
        }
        opaque_points
    }

    fn get_edges_points(&self) -> Vec<Point> {
        let opaque_points = self.get_opaque_points_grid();
        let mut edge_points = Vec::new();

        for (i, &is_opaque) in opaque_points.iter().flatten().enumerate() {
            if is_opaque {
                let x = i % self.size.0 as usize;
                let y = i / self.size.0 as usize;

                let left = opaque_points
                    .get(y)
                    .and_then(|row| row.get(x.wrapping_sub(1)));
                let right = opaque_points.get(y).and_then(|row| row.get(x + 1));
                let up = opaque_points
                    .get(y.wrapping_sub(1))
                    .and_then(|row| row.get(x));
                let down = opaque_points.get(y + 1).and_then(|row| row.get(x));

                if left != Some(&true)
                    || right != Some(&true)
                    || up != Some(&true)
                    || down != Some(&true)
                {
                    edge_points.push(Point {
                        x: x as u16,
                        y: y as u16,
                    });
                }
            }
        }

        edge_points
    }

    fn get_first_point_of_main_polygon(&self) -> Point {
        let grid = self.get_opaque_points_grid();
        for (y, row) in grid.iter().enumerate() {
            for (x, &is_opaque) in row.iter().enumerate() {
                if is_opaque {
                    return Point {
                        x: x as u16,
                        y: y as u16,
                    };
                }
            }
        }
        panic!("No opaque points found");
    }

    pub fn to_polygon(&self) -> (Polygon, Vec<Polygon>) {
        let mut holes: Vec<Polygon> = Vec::new();
        let edge_points = self.get_edges_points();

        let current_point = self.get_first_point_of_main_polygon();

        let polygon = get_polygon_from_point(current_point, edge_points.clone());

        // If there's some points that are in the edge_points list but not in the polygon list, it means that there's holes
        let mut remaining_points = edge_points
            .iter()
            .filter(|point| !polygon.contains(point))
            .cloned()
            .collect::<Vec<_>>();

        if edge_points.len() > polygon.len() {
            loop {
                if remaining_points.is_empty() {
                    break;
                }

                let hole = get_polygon_from_point(remaining_points[0], remaining_points.clone());

                remaining_points = remaining_points
                    .iter()
                    .filter(|point| !hole.contains(point))
                    .cloned()
                    .collect::<Vec<_>>();

                holes.push(hole);
            }
        }

        (polygon, holes)
    }
}

fn get_polygon_from_point(point: Point, all_points: Vec<Point>) -> Polygon {
    let mut polygon: Polygon = Vec::new();
    let mut current_point = point;
    loop {
        polygon.push(current_point);
        let right_point = all_points
            .iter()
            .find(|&point| Some(point) == current_point.right().as_ref());
        let right_up_point = all_points
            .iter()
            .find(|&point| Some(point) == current_point.right_up().as_ref());
        let right_down_point = all_points
            .iter()
            .find(|&point| Some(point) == current_point.right_down().as_ref());
        let up_point = all_points
            .iter()
            .find(|&point| Some(point) == current_point.up().as_ref());
        let down_point = all_points
            .iter()
            .find(|&point| Some(point) == current_point.down().as_ref());
        let left_point = all_points
            .iter()
            .find(|&point| Some(point) == current_point.left().as_ref());
        let left_up_point = all_points
            .iter()
            .find(|&point| Some(point) == current_point.left_up().as_ref());
        let left_down_point = all_points
            .iter()
            .find(|&point| Some(point) == current_point.left_down().as_ref());

        match (
            right_point,
            right_up_point,
            right_down_point,
            up_point,
            down_point,
            left_point,
            left_up_point,
            left_down_point,
        ) {
            (Some(point), _, _, _, _, _, _, _) if !polygon.contains(point) => {
                current_point = *point;
            }
            (_, Some(point), _, _, _, _, _, _) if !polygon.contains(point) => {
                current_point = *point;
            }
            (_, _, Some(point), _, _, _, _, _) if !polygon.contains(point) => {
                current_point = *point;
            }
            (_, _, _, Some(point), _, _, _, _) if !polygon.contains(point) => {
                current_point = *point;
            }
            (_, _, _, _, Some(point), _, _, _) if !polygon.contains(point) => {
                current_point = *point;
            }
            (_, _, _, _, _, Some(point), _, _) if !polygon.contains(point) => {
                current_point = *point;
            }
            (_, _, _, _, _, _, Some(point), _) if !polygon.contains(point) => {
                current_point = *point;
            }
            (_, _, _, _, _, _, _, Some(point)) if !polygon.contains(point) => {
                current_point = *point;
            }
            _ => break,
        }
    }

    polygon
}
impl From<Vec<u8>> for ImagePolygon {
    fn from(data: Vec<u8>) -> Self {
        Self::new(image::load_from_memory(&data).unwrap())
    }
}
