use super::*;
use num::{Float, NumCast};

type Point<T> = (T, T);

pub enum Intesection<T: Float> {
    Single(Point<T>),
    Double(Point<T>, Point<T>),
}

fn point_distance<T: Float>(p1: &Point<T>, p2: &Point<T>) -> T {
    ((p2.0 - p1.0).powi(2) + (p2.1 - p1.1).powi(2)).sqrt()
}

fn a_dist<T: Float>(c1: &Circle<T>, c2: &Circle<T>, distance: T) -> T {
    let _two: T = NumCast::from(2.0).unwrap();

    (c1.radius.powi(2) - c2.radius.powi(2) + distance.powi(2)) / (_two * distance)
}

fn h_find<T: Float>(c1: &Circle<T>, a: T) -> T {
    (c1.radius.powi(2) - a.powi(2)).sqrt()
}

fn intersection_point<T: Float>(c1: &Circle<T>, c2: &Circle<T>, distance: T, a: T) -> Point<T> {
    let x = c1.center.0 + a * (c2.center.0 - c1.center.0) / distance;
    let y = c1.center.1 + a * (c2.center.1 - c1.center.1) / distance;

    (x, y)
}

fn intersection_on_circles<T: Float>(
    c1: &Circle<T>,
    c2: &Circle<T>,
    p2: Point<T>,
    height: T,
    distance: T,
) -> (Point<T>, Point<T>) {
    let p3x_offset =
        |c1: &Circle<T>, c2: &Circle<T>, h: T, d: T| h * (c2.center.1 - c1.center.1) / d;
    let p3y_offset =
        |c1: &Circle<T>, c2: &Circle<T>, h: T, d: T| h * (c2.center.0 - c1.center.0) / d;
    let p3x1 = p2.0 + p3x_offset(c1, c2, height, distance);
    let p3x2 = p2.0 - p3x_offset(c1, c2, height, distance);
    let p3y1 = p2.1 - p3y_offset(c1, c2, height, distance);
    let p3y2 = p2.1 + p3y_offset(c1, c2, height, distance);

    ((p3x1, p3y1), (p3x2, p3y2))
}

pub fn circle_intersection<T: Float>(
    c1: &Circle<T>,
    c2: &Circle<T>,
) -> Result<Intesection<T>, &'static str> {
    let _zero: T = NumCast::from(0.0).unwrap();
    let distance = point_distance(&c1.center, &c2.center);
    let radius_sum = c1.radius + c2.radius;
    let radius_diff = (c1.radius - c2.radius).abs();

    if distance > radius_sum {
        return Err("No intersection");
    } else if distance < radius_diff {
        return Err("One circle is within another");
    } else if distance == _zero && c1.radius == c2.radius {
        return Err("Infinite number of solutions - circles are coincident");
    }

    let a = a_dist(&c1, &c2, distance);
    let h = h_find(&c1, a);
    let p2 = intersection_point(&c1, &c2, distance, a);
    let p3 = intersection_on_circles(&c1, &c2, p2, h, distance);

    if distance == radius_sum || distance == radius_diff {
        return Ok(Intesection::Single(p3.0));
    }

    Ok(Intesection::Double(p3.0, p3.1))
}
