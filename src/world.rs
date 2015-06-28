extern crate rand;

use rand::distributions::{IndependentSample, Range};
use entities::*;
use util::*;

pub struct World {
    asteroids: Vec<Asteroid>,
    projectiles: Vec<Projectile>,
    ship: Ship,
    paused: bool,
    viewport: Vector2<f64>,
    spawn_cooldown: f64,
    score: i32
}

impl World {
    pub fn new() -> World {
        World {
            asteroids: Vec::new(),
            projectiles: Vec::new(),
            ship: Ship::new(),
            paused: false,
            viewport: Vector2::new(0.0, 0.0),
            spawn_cooldown: 0.0,
            score: 0
        }
    }

    pub fn run(&mut self, dt: f64) {
        if self.paused {
            return;
        }

        if !self.ship.is_destroyed() {
            self.ship.move_ship(dt);
            if self.ship.fire_projectile(dt) {
                self.projectiles.push(Projectile::new(self.ship.position().clone()))
            }

            for projectile in self.projectiles.iter_mut() {
                projectile.simulate(dt);
            }
        }

        for asteroid in self.asteroids.iter_mut() {
            asteroid.simulate(dt);
        }

        self.spawn_asteroids(dt);
        self.handle_collisions();
        self.cleanup();
    }

    pub fn pause(&mut self, focus: bool) {
        self.paused = !focus;
    }

    pub fn update_viewport(&mut self, w: u32, h:u32) {
        self.viewport.x = w as f64;
        self.viewport.y = h as f64;
        self.ship.set_moveable_width(w);
    }

    pub fn renderables(&self) -> Vec<(&Renderable, Vector2<f64>)> {
        let mut result = Vec::new();
        if !self.ship.is_destroyed() {
            result.push((self.ship.renderable(), self.to_view(self.ship.position().clone())));
        }
        for projectile in self.projectiles.iter() {
            result.push((projectile.renderable(), self.to_view(projectile.position().clone())));
        }
        for asteroid in self.asteroids.iter() {
            result.push((asteroid.renderable(), self.to_view(asteroid.position().clone())));
        }
        result
    }

    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn reset(&mut self) {
        self.score = 0;
        self.ship = Ship::new();
        self.projectiles.clear();
        self.asteroids.clear();
    }

    pub fn get_ship(&mut self) -> &mut Ship {
        &mut self.ship
    }

    fn to_view(&self, position: Vector2<f64>) -> Vector2<f64> {
        let view_x = self.viewport.x / 2.0;
        let view_y = self.viewport.y - 50.0;
        
        Vector2::new(position.x + view_x , position.y + view_y)
    }

    fn spawn_asteroids(&mut self, dt: f64) {
        self.spawn_cooldown -= dt;

        if self.spawn_cooldown < 0.0 {
            let x_range = Range::new(-(self.viewport.x as f64) / 2.0, self.viewport.x / 2.0);
            let y_range = Range::new(-(self.viewport.y as f64) * 2.0, -(self.viewport.y));
            let mut rng = rand::thread_rng();

            let x = x_range.ind_sample(&mut rng);
            let y = y_range.ind_sample(&mut rng);

            self.asteroids.push(Asteroid::new(Vector2::new(x, y)));
            self.spawn_cooldown = 10.0 / (self.score + 1) as f64;
        }
    }


    fn handle_collisions(&mut self) {
        let mut destroyed_asteroid = None;
        let mut destroyed_projectile = None;

        for (j, projectile) in self.projectiles.iter().enumerate() {
            for (i, asteroid) in self.asteroids.iter().enumerate() {
                if intersect(projectile, asteroid) {
                    self.score += 1;
                    destroyed_asteroid = Some(i);
                    destroyed_projectile = Some(j);
                    break;
                }
            }

            if destroyed_projectile.is_some() {
                break;
            }
        }

        if destroyed_projectile.is_some() {
            self.projectiles.remove(destroyed_projectile.unwrap());
        }

        if destroyed_asteroid.is_some() {
            self.asteroids.remove(destroyed_asteroid.unwrap());
        }


        for asteroid in self.asteroids.iter() {
            if intersect(asteroid, &self.ship) {
                self.ship.destroy();
                self.projectiles.clear();
            }
        }
    }


    fn cleanup(&mut self) {
        let viewport_height = self.viewport.y;
        clean_collection(&mut self.projectiles, |projectile| {
            projectile.position().y < -(viewport_height as f64) 
        });

        clean_collection(&mut self.asteroids, |asteroid| {
            asteroid.position().y > 50.0
        });
    }
}


