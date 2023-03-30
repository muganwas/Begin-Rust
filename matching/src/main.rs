#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
}
#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let di_face = 4;
    let _game_some = Some(4);
    let _numbers: [Option<i32>; 5] = [Some(1), Some(2), None, Some(4), None];
    _da_game_if_let(_game_some);
    _da_game(di_face);
    _check_if_some(_numbers);
    println!("Penny equivalent in cents: {}", value_in_cents(Coin::Penny));
    println!(
        "Quarter equivalent in cents: {}",
        value_in_cents(Coin::Quarter(UsState::Alabama))
    );
    println!("Addition of 1 to 3: {:?}", plus_one(Some(3)));
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn _da_game(x: i32) {
    match x {
        4 => println!("Four"),
        7 => println!("Seven"),
        other => println!("Other: {}", other),
    }
}

fn _da_game_if_let(x: Option<i32>) {
    if let Some(i) = x {
        println!("Some is {}", i);
    }
}

fn _check_if_some(x: [Option<i32>; 5]) {
    let mut _x = 1;
    for i in x.iter() {
        if let Some(i) = i {
            println!("Some is {}", i);
        } else {
            _x += 1;
        }
    }
    println!("x is {}", _x);
}
