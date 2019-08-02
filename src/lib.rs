mod utils;

pub use utils::{Intesection,circle_intersection};

pub struct Circle {
    center: (f32, f32),
    radius: f32,
}

pub struct PolarCircle {
    center: (f32, f32),
    radius: f32,
}

impl Circle {
    pub fn new(x: f32, y: f32, radius: f32) -> Circle {
        Circle {
            center: (x, y),
            radius,
        }
    }

    pub fn to_polar(&self) -> PolarCircle {
        let center_dist = (self.center.0 * self.center.0 + self.center.1 * self.center.1).sqrt();
        let angle = (self.center.0 / center_dist).acos();
        PolarCircle::new((center_dist, angle), self.radius)
    }
}

impl PolarCircle {
    pub fn new(center: (f32, f32), radius: f32) -> PolarCircle {
        PolarCircle { center, radius }
    }

    pub fn to_cartesian(&self) -> Circle {
        let x = self.center.0 * self.center.1.cos();
        let y = self.center.0 * self.center.1.sin();
        Circle::new(x, y, self.radius)
    }
}

#[cfg(test)]
mod tests {
    use super::{Circle, PolarCircle};

    #[test]
    fn circle_create() {
        let expected_x = 1.0;
        let expected_y = 2.0;
        let expected_radius = 5.0;
        let c = Circle::new(expected_x, expected_y, expected_radius);
        assert_eq!(
            expected_x, c.center.0,
            "Wrong X coordinate after circle creation"
        );
        assert_eq!(
            expected_y, c.center.1,
            "Wrong Y coordinate after circle creation"
        );
        assert_eq!(
            expected_radius, c.radius,
            "Wrong radius after circle creation"
        )
    }

    #[test]
    fn polar_circle_create() {
        let expected_radius = 5.0;
        let expected_center = (2.0, 0.5);
        let c = PolarCircle::new(expected_center, expected_radius);
        assert_eq!(
            expected_radius, c.radius,
            "Wrong radius after circle creation"
        );
        assert_eq!(
            expected_center, c.center,
            "Wrong angle after circle creation"
        );
    }

    #[test]
    fn circle_to_polar_conversion() {
        let x = 2.0;
        let y = 2.0;
        let r = 6.0;
        let c = Circle::new(x, y, r);
        let pc = c.to_polar();
        let center_radius = (x * x + y * y).sqrt();

        assert_eq!(
            r, pc.radius,
            "Wrong radius after circle to polar circle conversion"
        );
        assert!(
            ((x / center_radius).acos() - pc.center.1).abs() < 0.01,
            "Wrong angle after circle to polar circle conversion"
        );
        assert!(
            ((y / center_radius).asin() - pc.center.1).abs() < 0.01,
            "Wrong angle after circle to polar circle conversion"
        );
    }

    #[test]
    fn polar_to_circle_conversion() {
        let angle = 0.3944444;
        let center_radius = 13.0;
        let radius = 6.0;
        let x: f32 = 12.0;
        let y: f32 = 5.0;
        let r: f32 = 6.0;
        let pc = PolarCircle::new((center_radius, angle), radius);
        let c = pc.to_cartesian();

        println!("{}", (x * x + y * y).sqrt());

        assert_eq!(r, c.radius, "Wrong radius after polar to circle conversion");
        assert_eq!(x, c.center.0.round(), "Wrong X coodrinate after conversion");
        assert_eq!(y, c.center.1.round(), "Wrong Y coodrinate after conversion");
    }
}
