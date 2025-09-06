use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, rad: f64) -> Self {
        Circle {
            center: (Point(x, y)),
            radius: (rad),
        }
    }
    pub fn area(&self) -> f64 {
        let p = PI;
        p * self.radius.powf(2.0)
    }
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }
    pub fn intersect(&self, c2: Circle) -> bool {
        let d: f64 = self.center.distance(c2.center);
        let min = c2.radius - self.radius;
        let max = c2.radius + self.radius;
        if d == min || d == max {
            true;
        }
        false
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(&self, p2: Point) -> f64 {
        let axis: f64 = (p2.0 - self.0).powi(2);
        let cord: f64 = (p2.1 - self.1).powi(2);
        (axis + cord).sqrt()
    }
}
