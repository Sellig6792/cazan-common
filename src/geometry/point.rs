use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Point {
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn right(&self) -> Option<Point> {
        self.x.checked_add(1).map(|x| Self { x, y: self.y })
    }

    pub fn left(&self) -> Option<Point> {
        self.x.checked_sub(1).map(|x| Self { x, y: self.y })
    }

    pub fn up(&self) -> Option<Point> {
        self.y.checked_sub(1).map(|y| Self { x: self.x, y })
    }

    pub fn down(&self) -> Option<Point> {
        self.y.checked_add(1).map(|y| Self { x: self.x, y })
    }

    pub fn right_up(&self) -> Option<Point> {
        self.right().and_then(|point| point.up())
    }

    pub fn right_down(&self) -> Option<Point> {
        self.right().and_then(|point| point.down())
    }

    pub fn left_up(&self) -> Option<Point> {
        self.left().and_then(|point| point.up())
    }

    pub fn left_down(&self) -> Option<Point> {
        self.left().and_then(|point| point.down())
    }
}
