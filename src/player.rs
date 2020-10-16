pub struct Player {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    move_speed: f32,
    dx: f32,
    dy: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            x: 30.0,
            y: 40.0,
            radius: 25.0,
            move_speed: 2.0,
            dx: 0.0,
            dy: 0.0,
        }
    }

    pub fn set_speed(&mut self, move_speed: f32) {
        self.move_speed = move_speed;
    }

    pub fn set_direction(&mut self, dx: f32, dy: f32) {
        self.dx = dx;
        self.dy = dy;
    }

    pub fn apply_movement(&mut self) {
        self.x += self.dx * self.move_speed;
        self.y += self.dy * self.move_speed;
    }
}
