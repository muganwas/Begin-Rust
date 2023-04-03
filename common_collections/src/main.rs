#[derive(Debug)]
enum _SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    let mut _v: Vec<i32> = Vec::new();
    let mut _v2 = vec![1, 2, 3];
    let _row = vec![
        _SpreadsheetCell::Int(3),
        _SpreadsheetCell::Text(String::from("blue")),
        _SpreadsheetCell::Float(10.12),
    ];
    _v.push(5);

    let _first: &i32 = &_v2[0];
    let _third: Option<&i32> = _v2.get(2);
    //_v2.push(345); // error: cannot borrow `_v2` as mutable because it is also borrowed as immutable
    println!("The first element is {}", _first);
    match _third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    _v2.push(345);
    // _add_to_each(&mut _v2, 3);
    // _iterate_over_vector(&_v2);
    for i in &_row {
        println!("{:?}", i);
    }
}

fn _add_to_each(v: &mut Vec<i32>, x: i32) {
    for i in v {
        *i += x;
    }
}

fn _iterate_over_vector(x: &Vec<i32>) {
    for i in x {
        println!("{}", i);
    }
}
