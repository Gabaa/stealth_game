use crate::nalgebra::Point2;

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
