use crate::nalgebra::Point2;
use crate::polygon::Polygon;

pub struct GameMap {
    pub obstacles: Vec<Polygon>,
    pub end_area: Polygon,
}

impl GameMap {
    pub fn new() -> Self {
        let obstacles = vec![
            Polygon::new(vec![
                Point2::new(0.0, 0.0),
                Point2::new(800.0, 0.0),
                Point2::new(800.0, 600.0),
                Point2::new(0.0, 600.0),
            ]),
            Polygon::new(vec![
                Point2::new(250.0, 250.0),
                Point2::new(325.0, 250.0),
                Point2::new(350.0, 350.0),
            ]),
            Polygon::new(vec![
                Point2::new(477.0, 142.0),
                Point2::new(541.0, 189.0),
                Point2::new(449.0, 328.0),
                Point2::new(374.0, 260.0),
                Point2::new(349.0, 211.0),
                Point2::new(428.0, 221.0),
                Point2::new(403.0, 162.0),
            ]),
        ];

        let end_area = Polygon::new(vec![
            Point2::new(700.0, 500.0),
            Point2::new(800.0, 500.0),
            Point2::new(800.0, 600.0),
            Point2::new(700.0, 600.0),
        ]);

        GameMap {
            obstacles,
            end_area,
        }
    }
}
