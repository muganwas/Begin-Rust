struct ImportantExcerpt<'a> {
    part: &'a str,
}
fn main() {
    let s1 = String::from("hello");
    let longer: &str;
    let s2 = String::from("hello world!");
    longer = longest(s1.as_str(), s2.as_str());
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("Novel text is {}", first_sentence);
    println!("The first sentence is {}", i.part);
    println!("The longest string is {}", longer);
}
fn longest<'a>(_s1: &'a str, _s2: &'a str) -> &'a str {
    if _s1.len() > _s2.len() {
        _s1
    } else {
        _s2
    }
}
