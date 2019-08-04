use std::fmt::Display;

use num::complex::Complex;
use num::traits::Float;
use num::NumCast;

use circle;
use circle::Intesection::{Double, Single};

fn dumb<T: Float + Display>(x: T, y: T) -> Complex<T> {
    let _one: T = NumCast::from(1.0).unwrap();
    let _two: T = NumCast::from(2.0).unwrap();
    let div = _one + x.powi(2) + _two * x + y.powi(2);
    let top_real = _one - x.powi(2) - y.powi(2);
    let top_imag = -_two * y;
    let y_adm = Complex::new(top_real / div, top_imag / div);

    y_adm.inv()
}

fn main() {
    let c1 = circle::Circle::new(-0.38_f32, 0.0, 0.621);
    let c2 = circle::Circle::new(0.5_f32, 0.0, 0.5);

    let intersection = circle::circle_intersection(&c1, &c2);

    if let Ok(int) = intersection {
        match int {
            Single(p) => println!("Intersection point: x - {}, y - {}", p.0, p.1),
            Double(p1, p2) => {
                println!("{:?} {:?}", p1, p2);
                println!("{:.2}", dumb(p1.0, p1.1));
                println!("{:.2}", dumb(p2.0, p2.1));
            }
        }
    }
}
