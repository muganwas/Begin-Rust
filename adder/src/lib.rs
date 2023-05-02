pub fn add_two(n: u32) -> u32 {
    n + 2
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 150 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

pub fn _greetings(_n: &str) -> String {
    format!("Hello {}!", _n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(110);
    }

    #[test]
    #[ignore = "expensive"]
    fn greetd() {
        let greeting = _greetings("Carol");
        assert!(
            greeting.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            greeting
        );
    }

    const LARGER: Rectangle = Rectangle {
        width: 8,
        height: 7,
    };
    const SMALLER: Rectangle = Rectangle {
        width: 5,
        height: 1,
    };

    #[test]
    fn larger_can_hold_smaller() {
        assert!(LARGER.can_hold(&SMALLER));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        assert!(!SMALLER.can_hold(&LARGER));
    }

    #[test]
    fn it_can_add_two() {
        assert_eq!(4, add_two(2)); // will fail
    }
}
