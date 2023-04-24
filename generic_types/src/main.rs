struct MixedPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> MixedPoint<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        &self.y
    }
}

impl MixedPoint<f32, f32> {
    fn _distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
fn main() {
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let largest = _largest(&number_list);
    println!("The largest number is {}", largest);
    let char_list = vec!['y', 'm', 'a', 'q', 'z'];
    let largest = _largest(&char_list);
    println!("The largest charactor is {}", largest);
    let _p = MixedPoint {
        x: String::from("You"),
        y: "Me",
    };
    let _p_2 = MixedPoint { x: "You", y: 1 };
    let _p_3 = MixedPoint { x: 1.0, y: 1.0 };
    let distance = _p_3._distance_from_origin();
    println!("Distance from origin is {}", distance);
    println!("x = {}, y = {}", _p.x(), _p.y());
    // _print_point(&_p);
    // _print_point(&_p_2);
}

fn _print_point<T: std::fmt::Display, U: std::fmt::Display>(point: &MixedPoint<T, U>) {
    println!("x = {}, y = {}", point.x, point.y);
}

fn _largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
