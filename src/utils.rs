use super::*;

type Point = (f32, f32);

pub enum Intesection {
    Single(Point),
    Double(Point, Point)
}

fn point_distance(p1: &Point, p2: &Point) -> f32 {
    ((p2.0 - p1.0).powi(2) + (p2.1 - p1.1).powi(2)).sqrt()
}

fn a_dist(c1: &Circle, c2: &Circle, distance: f32) -> f32 {
    (c1.radius.powi(2) - c2.radius.powi(2) + distance.powi(2)) / (2.0 * distance)
}

fn h_find(c1: &Circle, a: f32) -> f32 {
    (c1.radius.powi(2) - a.powi(2)).sqrt()
}

fn intersection_point(c1: &Circle, c2: &Circle, distance: f32, a: f32) -> Point {
    let x = c1.center.0 + a * (c2.center.0 - c1.center.0) / distance;
    let y = c1.center.1 + a * (c2.center.1 - c1.center.1) / distance;

    (x, y)
}

fn intersection_on_circles(c1: &Circle, c2: &Circle, p2: Point, height: f32, distance: f32) -> (Point, Point) {
    let p3x_offset = |c1: &Circle, c2: &Circle, h: f32, d: f32| h * (c2.center.1 - c1.center.1) / d;
    let p3y_offset = |c1: &Circle, c2: &Circle, h: f32, d: f32| h * (c2.center.0 - c1.center.0) / d;
    let p3x1 = p2.0 + p3x_offset(c1, c2, height, distance);
    let p3x2 = p2.0 - p3x_offset(c1, c2, height, distance);
    let p3y1 = p2.1 - p3y_offset(c1, c2, height, distance);
    let p3y2 = p2.1 + p3y_offset(c1, c2, height, distance);

    ((p3x1, p3y1), (p3x2, p3y2))
}

pub fn circle_intersection(c1: &Circle, c2: &Circle) -> Result<Intesection, &'static str> {
    let distance = point_distance(&c1.center, &c2.center);
    let radius_sum = c1.radius + c2.radius;
    let radius_diff = (c1.radius - c2.radius).abs();

    if distance > radius_sum {
        return Err("No intersection");
    }
    else if distance < radius_diff {
        return Err("One circle is within another");
    }
    else if distance == 0.0 && c1.radius == c2.radius {
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
