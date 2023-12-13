//use emums for Polymorphism 

enum Shape {
    Rectangle {width: u32, height: u32},
    Square(u32),
    Circle(f64),
    Triangle {width: u32, height: u32},
}

impl Shape {
    fn area(&self) -> f64 {
        match *self {
            Shape::Rectangle { width, height } => {
                let area = width * height;
                println!("{}", format_args!("Width: {}, Height: {}, Area: {}", width, height, area));
                area as f64
            },
            Shape::Square(ref s) => (s * s) as f64,
            Shape::Circle(ref r) => 3.14 * (r * r),
            Shape::Triangle {width, height} => (width * height / 2) as f64,
        }
    }
}

fn main() {
    let r = Shape::Rectangle { width: 10, height: 70 };
    let s = Shape::Square(10);
    let c = Shape::Circle(4.5);
    let t = Shape::Triangle { width: 10, height: 70 };

    let ar = r.area();
    println!("Area of Rectangle {}", ar);

    let aq = s.area();
    println!("Area of Square {}", aq);

    let ac = c.area();
    println!("Area of Circle {}", ac);

    let tr = t.area();
    println!("Area of Triangle is {}", tr);

}
    