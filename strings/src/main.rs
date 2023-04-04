fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hello ");
    let s2 = String::from("world");
    let s3 = s1 + &s2;
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3); // inbuilt refrences
    println!("{}", s);
    _iterate_string_by_bytes(&s);
    _iterate_string_by_chars(&s);
}

fn _iterate_string_by_chars(x: &String) {
    for c in x.chars() {
        println!("{}", c);
    }
}

fn _iterate_string_by_bytes(x: &String) {
    for c in x.bytes() {
        println!("{}", c);
    }
}
