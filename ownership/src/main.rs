fn main() {
    let mut s = String::from("stee");
    let _str = s.clone();
    println!(
        "this is the initial string {} this is the clone {}",
        s, _str
    );
    try_modding(&mut s);
}

fn try_modding(s: &mut String) {
    s.push_str(" world");
    println!("{}", s);
}
