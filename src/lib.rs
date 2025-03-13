use raylib::prelude::*;
use rand::{seq::SliceRandom, Rng};

pub struct Particle {
    pub position: Vector2,
    pub velocity: Vector2,
    pub acceleration: Vector2,
}

impl Particle {
    pub fn new(position: Vector2, velocity: Vector2, acceleration: Vector2) -> Self {
        Particle {
            position,
            velocity,
            acceleration,
        }
    }

    pub fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.acceleration *= 0.0;
    }

    pub fn apply_force(&mut self, force: Vector2) {
        self.acceleration += force;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_particle() {
        let velocity = Vector2::new(1.0, 2.0);
        let acceleration = Vector2::new(3.0, 4.0);
        let position = Vector2::new(5.0, 6.0);
        let result = Particle::new(position, velocity, acceleration);
        assert_eq!(result.velocity, velocity);
        assert_eq!(result.acceleration, acceleration);
        assert_eq!(result.position, position);
    }

    #[test]
    fn apply_force() {
        let mut particle = Particle::new(Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0));
        let force = Vector2::new(1.0, 2.0);
        particle.apply_force(force);
        assert_eq!(particle.acceleration, force);
    }

    #[test]
    fn update() {
        let mut particle = Particle::new(Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0));
        particle.apply_force(Vector2::new(1.0, 2.0));
        particle.update();
        assert_eq!(particle.velocity, Vector2::new(1.0, 2.0));
        assert_eq!(particle.position, Vector2::new(1.0, 2.0));
        assert_eq!(particle.acceleration, Vector2::new(0.0, 0.0));
    }
}

const PERM_SIZE: usize = 256;

/// Perlin noise generator
pub struct Perlin {
    perm: [usize; PERM_SIZE * 2], // Double for overflow handling
    gradients: [(f64, f64); PERM_SIZE],
}

impl Perlin {
    /// Create a new Perlin noise generator
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut perm: Vec<usize> = (0..PERM_SIZE).collect();
        perm.shuffle(&mut rng);

        let mut gradients = [(0.0, 0.0); PERM_SIZE];
        for i in 0..PERM_SIZE {
            let angle = rng.r#gen::<f64>() * std::f64::consts::PI * 2.0;
            // Random unit vectors
            gradients[i] = (angle.cos(), angle.sin());
        }

        let mut perm_table = [0; PERM_SIZE * 2];
        for i in 0..PERM_SIZE {
            perm_table[i] = perm[i];
            // Duplicate for overflow handling
            perm_table[i + PERM_SIZE] = perm[i];
        }

        Self { perm: perm_table, gradients }
    }

    /// Fade function as defined by Ken Perlin. This eases coordinate values
    fn fade(t: f64) -> f64 {
        t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
    }

    /// Linear interpolation
    fn grad(&self, hash: usize, x: f64, y: f64) -> f64 {
        let (gx, gy) = self.gradients[hash % PERM_SIZE];
        // Dot product
        gx * x + gy * y
    }

    /// 2D Perlin noise
    pub fn noise(&self, x: f64, y: f64) -> f64 {
        let x0 = x.floor() as isize;
        let y0 = y.floor() as isize;
        let x1 = x0 + 1;
        let y1 = y0 + 1;

        let dx = x - x0 as f64;
        let dy = y - y0 as f64;

        let u = Self::fade(dx);
        let v = Self::fade(dy);

        let g00 = self.grad(self.perm[(x0 as usize + self.perm[y0 as usize % PERM_SIZE]) % PERM_SIZE], dx, dy);
        let g10 = self.grad(self.perm[(x1 as usize + self.perm[y0 as usize % PERM_SIZE]) % PERM_SIZE], dx - 1.0, dy);
        let g01 = self.grad(self.perm[(x0 as usize + self.perm[y1 as usize % PERM_SIZE]) % PERM_SIZE], dx, dy - 1.0);
        let g11 = self.grad(self.perm[(x1 as usize + self.perm[y1 as usize % PERM_SIZE]) % PERM_SIZE], dx - 1.0, dy - 1.0);

        let nx0 = g00 + u * (g10 - g00);
        let nx1 = g01 + u * (g11 - g01);
        let n = nx0 + v * (nx1 - nx0);

        n
    }
}
