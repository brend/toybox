use raylib::prelude::*;

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
