use circle;

use circle::Intesection::{Single, Double};

fn dumb(x: f32, y: f32) {
    let div = (1.0 + x.powi(2) + 2.0 * x + y.powi(2));
    let top_real = (1.0 - x.powi(2) - y.powi(2));
    let top_imag = (-2.0 * y);
    let y_adm = top_real / div;
    let b_adm = top_imag / div;

    println!("{} {}", y_adm, b_adm);
}

fn main() {
    let c1 = circle::Circle::new(-0.38, 0.0, 0.621);
    let c2 = circle::Circle::new(0.5, 0.0, 0.5);

    let intersection = circle::circle_intersection(&c1, &c2);

    if let Ok(int) = intersection {
        match int {
            Single(p) => println!("Intersection point: x - {}, y - {}", p.0, p.1),
            Double(p1, p2) => {
                println!("{:?} {:?}", p1, p2);
                dumb(p1.0, p1.1);
                dumb(p2.0, p2.1);
            }
        }
    }
}
