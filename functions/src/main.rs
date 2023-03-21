fn main() {
    another_function(10);
    let x = five();
    println!("Another x: {x}");
    let plus_uno = plus_one(x);
    println!("x + 1 = {plus_uno}");
}

fn another_function(x: u32) {
    println!("The value of x is: {x}");
}
fn five() -> u32 {
    5
}
fn plus_one(x: u32) -> u32 {
    x + 1
}
