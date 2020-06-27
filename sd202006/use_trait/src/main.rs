struct RightTriangle {
    base: f64,
    perpendicular: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

trait GeoCalculator {
    fn area(&self) -> f64;
    fn length(&self) -> f64;
}

impl GeoCalculator for RightTriangle {
    fn area(&self) -> f64 {
        (self.base * self.perpendicular) * 0.5
    }
    fn length(&self) -> f64 {
        self.base + self.perpendicular
            + (self.base.powi(2) + self.perpendicular.powi(2)).sqrt()
    }
}

impl GeoCalculator for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn length(&self) -> f64 {
        (self.width + self.height) * 2.0
    }
}

fn printval<T: GeoCalculator>(poly: &T) {
    println!("{}", poly.area());
    println!("{}", poly.length());
}

fn main() {
    let tri = RightTriangle {
        base: 3.0,
        perpendicular: 4.0,
    };
    printval(&tri);

    let rec = Rectangle {
        width: 3.0,
        height: 4.0,
    };
    printval(&rec);
}
