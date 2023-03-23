struct User {
    active: bool,
    username: String,
    password: String,
    login: u64,
}

fn main() {
    let mut s = String::from("stee");
    let mut _str = s.clone();
    let _str_1 = &s;

    let first_user = User {
        active: true,
        username: String::from("Kaganwa"),
        password: String::from("xyz"),
        login: 1,
    };

    return_user(first_user);

    println!(
        "this is the initial string {} this is the clone {} and another {}",
        s, _str, _str_1
    );
    let _str_2 = &mut s;
    try_modding(&mut s);
    let string_to_slice = "Hello world!";
    let _first_word = first_word(&string_to_slice);
    let _second_word = second_word(&string_to_slice);
    println!("first word: {}, second word: {}", _first_word, _second_word);
}

fn try_modding(s: &mut String) {
    s.push_str(" world");
    println!("{}", s);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            let _index_plus = i + 1;
            let without_first_word = &bytes[_index_plus..];
            for (_in, &item_1) in without_first_word.iter().enumerate() {
                if item_1 == b' ' {
                    return &s[_index_plus.._in];
                }
            }
            return &s[_index_plus..];
        }
    }
    return &s[..];
}

fn return_user(user: User) -> (bool, String, String, u64) {
    let user = (user.active, user.username, user.password, user.login);
    user
}
