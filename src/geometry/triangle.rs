use crate::geometry::Point;
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Triangle(pub Point, pub Point, pub Point);

impl IntoIterator for Triangle {
    type Item = Point;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.0, self.1, self.2].into_iter()
    }
}

impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        let mut self_points = vec![self.0, self.1, self.2];
        let mut other_points = vec![other.0, other.1, other.2];
        self_points.sort();
        other_points.sort();
        self_points == other_points
    }
}
