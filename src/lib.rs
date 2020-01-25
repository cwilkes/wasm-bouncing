
use std::fmt;
use itertools::free::join;

pub struct Ball {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
}

impl Ball {
    fn advance(&mut self, width: f32, height: f32) {
        self.x += self.dx;
        self.y += self.dy;
        if self.x < 0.0 {
            self.x = -self.x;
            self.dx = -self.dx;
        }
        if self.y < 0.0 {
            self.y = -self.y;
            self.dy = -self.dy;
        }
        if self.x >= width {
            self.x = width - (self.x - width);
            self.dx = -self.dx;
        }
        if self.y >= height {
            self.y = height - (self.y - height);
            self.dy = -self.dy;
        }
    }
}

impl fmt::Display for Ball {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})_<{},{}>", self.x, self.y, self.dx, self.dy)
    }
}

pub struct Universe {
    width: f32,
    height: f32,
    projectiles: Vec<Ball>,
}

impl Universe {
    pub fn advance(self: &mut Self) {
        for ball in &mut self.projectiles {
            ball.advance(self.width, self.height);
        }
    }

    pub fn new() -> Universe {
        let mut balls: Vec<Ball> = Vec::new();
        balls.push(Ball { x: 0.0, y: 0.0, dx: 5.0, dy: -7.0 });
        balls.push(Ball { x: 50.0, y: 70.0, dx: -2.0, dy: 3.0 });
        Universe {
            width: 100.0,
            height: 200.0,
            projectiles: balls,
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&join(&self.projectiles[..], &","))?;
        Ok(())
    }
}

