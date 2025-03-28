struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f32> {
    pub fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl Point<f64> {
    pub fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl Point<i32> {
    pub fn distance_from_origin(&self) -> f32 {
        ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt()
    }
}

impl Point<u32> {
    pub fn distance_from_origin(&self) -> f32 {
        ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt()
    }
}

fn main() {
    let p: Point<i32> = Point { x: 5, y: 10 };
    println!("p.x = {} p.y = {}", p.x(), p.y());
    println!("distance from origin: {}", p.distance_from_origin());

    let p2: Point<f64> = Point { x: 5.0, y: 10.0 };
    println!("p2.x = {} p2.y= {}", p2.x(), p2.y());
    println!("distance from origin is {}", p2.distance_from_origin());

    let p3: Point<f32> = Point { x: 5.0, y: 10.0 };
    println!("p3.x = {} p3.y = {}", p3.x(), p3.y());
    println!("distance from origin is {}", p3.distance_from_origin());
}
