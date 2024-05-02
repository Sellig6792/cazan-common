use crate::geometry::*;
use image;
use image::GenericImageView;

pub type Grid<T> = Vec<Vec<T>>;

pub struct ImageEdgesParser {
    image: image::DynamicImage,
    size: (u16, u16),
}

impl ImageEdgesParser {
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

    pub fn as_polygon(&self) -> Polygon {
        let mut points = Vec::new();
        let edge_points = self.get_edges_points();

        let mut current_point = self.get_first_point_of_main_polygon();

        loop {
            points.push(current_point);
            let right_point = edge_points
                .iter()
                .find(|&point| Some(point) == current_point.right().as_ref());
            let right_up_point = edge_points
                .iter()
                .find(|&point| Some(point) == current_point.right_up().as_ref());
            let right_down_point = edge_points
                .iter()
                .find(|&point| Some(point) == current_point.right_down().as_ref());
            let up_point = edge_points
                .iter()
                .find(|&point| Some(point) == current_point.up().as_ref());
            let down_point = edge_points
                .iter()
                .find(|&point| Some(point) == current_point.down().as_ref());
            let left_point = edge_points
                .iter()
                .find(|&point| Some(point) == current_point.left().as_ref());
            let left_up_point = edge_points
                .iter()
                .find(|&point| Some(point) == current_point.left_up().as_ref());
            let left_down_point = edge_points
                .iter()
                .find(|&point| Some(point) == current_point.left_down().as_ref());

            // log(&format!("right_point: {:?}\nright_up_point: {:?}\nright_down_point: {:?}\nup_point: {:?}\ndown_point: {:?}\nleft_point: {:?}\nleft_up_point: {:?}\nleft_down_point: {:?}", right_point, right_up_point, right_down_point, up_point, down_point, left_point, left_up_point, left_down_point));
            // Check if the point is not the last point of the polygon and if it is not already in the list
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
                (Some(point), _, _, _, _, _, _, _) if !points.contains(point) => {
                    current_point = *point;
                }
                (_, Some(point), _, _, _, _, _, _) if !points.contains(point) => {
                    current_point = *point;
                }
                (_, _, Some(point), _, _, _, _, _) if !points.contains(point) => {
                    current_point = *point;
                }
                (_, _, _, Some(point), _, _, _, _) if !points.contains(point) => {
                    current_point = *point;
                }
                (_, _, _, _, Some(point), _, _, _) if !points.contains(point) => {
                    current_point = *point;
                }
                (_, _, _, _, _, Some(point), _, _) if !points.contains(point) => {
                    current_point = *point;
                }
                (_, _, _, _, _, _, Some(point), _) if !points.contains(point) => {
                    current_point = *point;
                }
                (_, _, _, _, _, _, _, Some(point)) if !points.contains(point) => {
                    current_point = *point;
                }
                _ => break,
            }
        }
        points
    }
}

impl From<Vec<u8>> for ImageEdgesParser {
    fn from(data: Vec<u8>) -> Self {
        Self::new(image::load_from_memory(&data).unwrap())
    }
}
