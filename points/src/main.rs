struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, another_point: Point) -> f64 {
        let common_x = f64::powi(self.x * another_point.x, 2);
        let common_y = f64::powi(self.y * another_point.y, 2);
        f64::sqrt(common_y + common_x)
    }
}

fn main() {
    let point1 = Point{x: 1.0, y: 2.0};
    let point2 = Point{x: 6.0, y: 8.0};
    println!("Distance above point1 and point2 is: {:?} dots", point1.distance(point2));
}
