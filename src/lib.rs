extern crate wasm_bindgen;

use std::fmt;
use itertools::free::join;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub struct Ball {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
}

#[wasm_bindgen]
impl Ball {
    fn advance(&mut self, width: f32, height: f32) -> Ball {
        let mut dx = self.dx;
        let mut dy = self.dy;
        let mut x = self.x + dx;
        let mut y = self.y + dy;
        if x < 0.0 {
            x = -x;
            dx = -dx;
        }
        if y < 0.0 {
            y = -y;
            dy = -dy;
        }
        if x >= width {
            x = width - (x - width);
            dx = -dx;
        }
        if y >= height {
            y = height - (y - height);
            dy = -dy;
        }
        Ball { x: x, y: y, dx: dx, dy: dy }
    }
}

impl fmt::Display for Ball {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})_<{},{}>", self.x, self.y, self.dx, self.dy)
    }
}

#[derive(Debug)]
pub struct Universe {
    width: f32,
    height: f32,
    projectiles: Vec<Ball>,
}

#[wasm_bindgen]
impl Universe {
    pub fn advance(self: &mut Self) -> Universe {
        let mut positions: Vec<Ball> = Vec::new();
        for ball in &mut self.projectiles {
            positions.push(ball.advance(self.width, self.height));
        }
        Universe { width: self.width, height: self.height, projectiles: positions }
    }

    pub fn new() -> Universe {
        let mut projectiles: Vec<Ball> = Vec::new();
        projectiles.push(Ball { x: 0.0, y: 0.0, dx: 6.0, dy: -7.0 });
        projectiles.push(Ball { x: 50.0, y: 70.0, dx: -2.0, dy: 3.0 });
        Universe {
            width: 100.0,
            height: 200.0,
            projectiles: projectiles,
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&join(&self.projectiles[..], &","))?;
        Ok(())
    }
}

