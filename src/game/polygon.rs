use std::cmp::Ordering;

use ggez::{graphics::Rect, nalgebra::Point2};

pub struct Polygon {
    pub verts: Vec<Point2<f32>>,
}

impl Polygon {
    pub fn new(verts: Vec<Point2<f32>>) -> Self {
        // assert!(verts.len() >= 3);
        Polygon { verts }
    }

    pub fn edge_iter(&self) -> EdgeIterator {
        let newvec = self.verts.clone();
        EdgeIterator::new(newvec)
    }

    pub fn contains(&self, mouse_pos: Point2<f32>) -> bool {
        self.bounding_box().contains(mouse_pos)
    }

    pub fn bounding_box(&self) -> Rect {
        let min_x = self
            .verts
            .iter()
            .map(|point| point.x)
            .min_by(|x1, x2| x1.partial_cmp(x2).unwrap_or(Ordering::Equal))
            .unwrap();
        let max_x = self
            .verts
            .iter()
            .map(|point| point.x)
            .max_by(|x1, x2| x1.partial_cmp(x2).unwrap_or(Ordering::Equal))
            .unwrap();
        let min_y = self
            .verts
            .iter()
            .map(|point| point.y)
            .min_by(|y1, y2| y1.partial_cmp(y2).unwrap_or(Ordering::Equal))
            .unwrap();
        let max_y = self
            .verts
            .iter()
            .map(|point| point.y)
            .max_by(|y1, y2| y1.partial_cmp(y2).unwrap_or(Ordering::Equal))
            .unwrap();

        Rect {
            x: min_x,
            y: min_y,
            w: max_x - min_x,
            h: max_y - min_y,
        }
    }
}

pub struct EdgeIterator {
    verts: Vec<Point2<f32>>,
    i: usize,
}

impl EdgeIterator {
    fn new(verts: Vec<Point2<f32>>) -> Self {
        EdgeIterator { verts, i: 0 }
    }
}

impl Iterator for EdgeIterator {
    type Item = (Point2<f32>, Point2<f32>);

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.verts.len();
        if self.i >= n {
            return None;
        }
        let a = self.verts[self.i];
        let b = self.verts[(self.i + 1) % n];

        self.i += 1;

        Some((a, b))
    }
}
