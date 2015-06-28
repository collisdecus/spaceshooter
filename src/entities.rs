use util::{Vector2, Rect};
use std::f64;

pub trait Entity {
    fn renderable(&self) -> &Renderable;
    fn position(&self) -> &Vector2<f64>;
}

pub struct Renderable {
    pub color: [f32; 4],
    pub polygon: Vec<[f64; 2]>
}

impl Renderable {
    fn new(color: [f32; 4], polygon: Vec<[f64; 2]>) -> Renderable {
        Renderable {
            color: color,
            polygon: polygon
        }
    }

    pub fn bounding_box(&self) -> Rect<f64> {
        let mut left = f64::MAX;
        let mut right = f64::MIN;
        let mut top = f64::MAX;
        let mut bottom = f64::MIN;

        for vertex in self.polygon.iter() {
            left = if vertex[0] < left { vertex[0] } else { left };
            top = if vertex[1] < top { vertex[1] } else { top };
            right = if vertex[0] > right { vertex[0] } else { right };
            bottom = if vertex[1] > bottom { vertex[1] } else { bottom };
        }

        Rect::new(top, bottom, left, right)
    }
}


pub struct Asteroid {
    position: Vector2<f64>,
    renderable: Renderable
}

impl Asteroid {
    pub fn new(position: Vector2<f64>) -> Asteroid {
        Asteroid {
            position: position,
            renderable: Renderable::new(
                [0.7, 0.7, 0.7, 1.0],
                vec![[-8.0, -8.0], [-8.0, 8.0], [8.0, 8.0], [8.0, -8.0]])
        }
    }

    pub fn simulate(&mut self, dt: f64) {
        self.position.y += dt * 100.0;
    }

}

impl Entity for Asteroid {
    fn renderable(&self) -> &Renderable {
        &self.renderable
    }

    fn position(&self) -> &Vector2<f64> {
        &self.position
    }
}

pub struct Projectile {
    position: Vector2<f64>,
    renderable: Renderable
}

impl Projectile {
    pub fn new(position: Vector2<f64>) -> Projectile {
        Projectile {
            position: position,
            renderable: Renderable::new(
                [1.0, 0.0, 0.0, 1.0],
                vec![[-2.0, -2.0], [-2.0, 2.0], [2.0, 2.0], [2.0, -2.0]])
        }
    }

    pub fn simulate(&mut self, dt: f64) {
        self.position.y -= dt * 1000.0;
    }

    pub fn renderable(&self) -> &Renderable {
        &self.renderable
    }
}

impl Entity for Projectile {
    fn renderable(&self) -> &Renderable {
        &self.renderable
    }

    fn position(&self) -> &Vector2<f64> {
        &self.position
    }
}

pub struct Ship {
    position: Vector2<f64>,
    thrust_left: bool,
    thrust_right: bool,
    firing: bool,
    cooldown: f64,
    destroyed: bool,
    moveable_width: f64,
    renderable: Renderable
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            position: Vector2::new(0.0, 0.0),
            thrust_left: false,
            thrust_right: false,
            firing: false,
            destroyed: false,
            moveable_width: 0.0,
            cooldown: -1.0,
            renderable: Renderable::new(
                [0.2, 0.2, 0.8, 1.0],
                vec![[-20.0, 20.0], [0.0, -20.0], [20.0, 20.0]])
        }
    }

    pub fn thrust_left(&mut self, enable: bool) {
        self.thrust_left = enable;
    }

    pub fn thrust_right(&mut self, enable: bool) {
        self.thrust_right = enable;
    }

    pub fn fire(&mut self, enable: bool) {
        self.firing = enable;
    }

    pub fn move_ship(&mut self, dt: f64) {
        if self.thrust_right {
            self.position.x += dt * 200.0;
        }

        if self.position.x < -self.moveable_width {
            self.position.x = -self.moveable_width;
        }

        if self.thrust_left {
            self.position.x -= dt * 200.0;
        }

        if self.position.x > self.moveable_width {
            self.position.x = self.moveable_width;
        }
    }

    pub fn fire_projectile(&mut self, dt: f64) -> bool {
        self.cooldown -= dt;
        if self.firing && self.cooldown < 0.0 {
            self.cooldown = 0.6;
            true
        } else {
            false
        }
    }

    pub fn destroy(&mut self) {
        self.destroyed = true;
    }

    pub fn is_destroyed(&self) -> bool {
        self.destroyed
    }

    pub fn set_moveable_width(&mut self, moveable: u32) {
        self.moveable_width = moveable as f64 / 2.0;
    }
}

impl Entity for Ship {
    fn renderable(&self) -> &Renderable {
        &self.renderable
    }

    fn position(&self) -> &Vector2<f64> {
        &self.position
    }
}