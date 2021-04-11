use crate::tuple::Tuple;

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

impl Projectile {
    fn update(&self, env: &Environment, dtime: f64) -> Self {
        let position = self.position + (self.velocity * dtime);
        let accel = (env.gravity + env.wind) * dtime;
        let velocity = self.velocity + accel;

        Projectile { position, velocity }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update() {
        let position = Tuple::origin();
        let velocity = Tuple::vector(1., 1., 0.);
        let proj = Projectile { position, velocity };

        let gravity = Tuple::vector(0., -0.5, 0.);
        let wind = Tuple::vector(-0.1, 0., 0.);
        let env = Environment { gravity, wind };

        let next = proj.update(&env, 1.0);
        assert_eq!(next.position, Tuple::point(1., 1., 0.));
        assert_eq!(next.velocity, Tuple::vector(0.9, 0.5, 0.));

        let next = proj.update(&env, 2.0);
        assert_eq!(next.position, Tuple::point(2., 2., 0.));
        assert_eq!(next.velocity, Tuple::vector(0.8, 0.0, 0.));
    }
}
