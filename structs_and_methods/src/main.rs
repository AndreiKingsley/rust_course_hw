use std::f64::consts::PI;

struct Point {
    x: f64,
    y: f64,
}

struct Rect {
    upper_left_vertex: Point,
    lower_right_vertex: Point,
}

struct Circle {
    center: Point,
    radius: f64,
}

enum Figure {
    Rect(Rect),
    Circle(Circle),
}

fn distance(a: &Point, b: &Point) -> f64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    (dx * dx + dy * dy).sqrt()
}

impl Rect {
    fn contains(&self, p: &Point) -> bool {
        if self.upper_left_vertex.x > p.x || self.lower_right_vertex.x < p.x {
            return false;
        }
        if self.upper_left_vertex.y > p.y || self.lower_right_vertex.y < p.y {
            return false;
        }
        true
    }

    fn area(&self) -> f64 {
        let dx = self.lower_right_vertex.x - self.upper_left_vertex.x;
        let dy = self.lower_right_vertex.y - self.upper_left_vertex.y;
        dx * dy
    }
}

impl Circle {
    fn contains(&self, p: &Point) -> bool {
        let dist_to_center = distance(p, &self.center);
        dist_to_center <= self.radius
    }

    fn area(&self) -> f64 {
        return PI * self.radius * self.radius;
    }
}

impl Figure {
    fn contains(&self, p: &Point) -> bool {
        return match self {
            Figure::Rect(rect) => rect.contains(p),
            Figure::Circle(circle) => circle.contains(p)
        };
    }
}

fn main() {
    // I use a coordinate system where y axis is downward

    println!("1. Rect((1.1, 2.1), (19.22, 19.91)) contains Point(1.0, 2.0):");
    println!(
        "{}",
        Rect {
            upper_left_vertex: Point { x: 1.0, y: 2.1 },
            lower_right_vertex: Point { x: 19.22, y: 19.91 },
        }.contains(
            &Point { x: 1.0, y: 2.0 }
        )
    );

    println!("2. Rect((-11.11, 22.22), (33.33, 44.55)) contains Point(12.0, 44.0):");
    println!(
        "{}",
        Rect {
            upper_left_vertex: Point { x: -11.11, y: 22.22 },
            lower_right_vertex: Point { x: 33.33, y: 44.55 },
        }.contains(
            &Point { x: 12.0, y: 44.0 }
        )
    );

    println!("3. Circle((0.0, 0.0), 150.0) contains Point(0.0, 150.0):");
    println!(
        "{}",
        Circle {
            center: Point { x: 0.0, y: 0.0 },
            radius: 150.0,
        }.contains(
            &Point { x: 0.0, y: 150.0 }
        )
    );
    println!("4. Circle((-100.0, -100000.0), 0.2345) contains Point(100.100, 100.500):");
    println!(
        "{}",
        Circle {
            center: Point { x: -100.0, y: -100000.0 },
            radius: 0.2345,
        }.contains(
            &Point { x: 100.100, y: 100.500 }
        )
    );

    println!("5. Area of Rect((12.0, 25.71), (13.5, 29.71)):");
    println!(
        "{}",
        Rect {
            upper_left_vertex: Point { x: 12.0, y: 25.71 },
            lower_right_vertex: Point { x: 13.5, y: 29.71 },
        }.area()
    );

    println!("6. Area of Circle((-1748.771199, -121.2119), 10.0):");
    println!(
        "{}",
        Circle {
            center: Point { x: -1748.771199, y: -121.2119 },
            radius: 10.0,
        }.area()
    );

    println!("7. Figure(Rect((-0.1111, -0.2222), (14.88, 13.37))) contains Point(-1.0, -20.0):");
    println!(
        "{}",
        Figure::Rect(Rect {
            upper_left_vertex: Point { x: -0.1111, y: -0.2222 },
            lower_right_vertex: Point { x: 14.88, y: 13.37 },
        }).contains(
            &Point { x: -1.0, y: -20.0 }
        )
    );

    println!("8. Figure(Circle((12.3, 13.3), 42424242.42)) contains Point(0.0, 1000.1):");
    println!(
        "{}",
        Figure::Circle(Circle {
            center: Point { x: 12.3, y: 13.3 },
            radius: 42424242.42,
        }).contains(
            &Point { x: 0.0, y: 1000.1 }
        )
    );
}
