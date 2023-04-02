pub fn add(left: usize, right: usize) -> usize {
    left + right
}
#[allow(dead_code)]
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    pub enum Apetizer {
        Soup,
        Salad,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

mod customer {
    use crate::back_of_house::*;
    pub fn _eat_at_restaurant() {
        // Absolute path
        crate::front_of_house::_add_to_waitlist();
        let mut _breakfast = Breakfast::summer("Barley");
        let _soup = Apetizer::Soup;
        let _salad = Apetizer::Salad;
        _breakfast.toast = String::from("Wheat");
        println!("I'd like {} toast please", _breakfast.toast);
        //println!("I got {}", _breakfast.seasonal_fruit);
        //_breakfast.seasonal_fruit = String::from("Banana");
    }
}

mod front_of_house {
    pub fn _add_to_waitlist() {
        let _sum = super::add(5, 10);
        println!("sum: {}", _sum);
    }
    pub mod hosting {
        fn _seat_at_table() {}
    }

    mod serving {
        fn _take_order() {}

        fn _serve_order() {}

        fn _take_payment() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
