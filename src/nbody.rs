use rand::{thread_rng, Rng};

// Higher value of G for better attraction
const G: f64 = 6.674e0;
const SOFTENING: f64 = 3e4;

#[derive(Debug, PartialEq)]
pub struct Particle {
    pub mass: f64,
    pub position: [f64; 2],
    pub color: [f32; 4],
    velocity: [f64; 2],
    force: [f64; 2],
}

impl Particle {
    fn update(&mut self, dt: f64) {
        self.velocity[0] += (self.force[0] / self.mass) * dt;
        self.velocity[1] += (self.force[1] / self.mass) * dt;
        self.position[0] += self.velocity[0] * dt;
        self.position[1] += self.velocity[1] * dt;
    }

    fn reset_force(&mut self) {
        self.force[0] = 0.0;
        self.force[1] = 0.0;
    }

    fn add_force(&mut self, other: &Particle) {
        let delta_x = other.position[0] - self.position[0];
        let delta_y = other.position[1] - self.position[1];
        let distance = (delta_x * delta_x + delta_y * delta_y).sqrt();
        let f = (G * self.mass * other.mass) / (distance * distance + SOFTENING * SOFTENING);
        self.force[0] += f * delta_x / distance;
        self.force[1] += f * delta_y / distance;
    }
}

pub fn random_particle() -> Particle {
    let mut rng = thread_rng();

    return Particle {
        mass: rng.gen_range(10000000.0, 250000000.0),
        position: [rng.gen_range(200.0, 600.0), rng.gen_range(200.0, 400.0)],
        color: rng.gen(),
        force: [0.0, 0.0],
        velocity: [0.0, 0.0]
    }
}

pub fn step(particles: &mut Vec<Particle>, dt: f64) {
    for particle in particles.iter_mut() {
        particle.update(dt);
        particle.reset_force();
    }

    let mut iterator = particles.iter_mut();

    while let Some(particle) = iterator.next() {
        for other_particle in &mut iterator {
            particle.add_force(other_particle);
            other_particle.add_force(particle);
        }
    }
}
