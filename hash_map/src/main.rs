use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    let key_1 = String::from("Blue");
    let key_2 = String::from("Yellow");
    scores.insert(key_1, 10);
    scores.insert(key_2, 50);
    let blue_score = scores.get("Blue").copied().unwrap_or(0);
    println!("{:?}", blue_score);
    //println!("{key_1}"); // error: cannot find value `key_1` in this scope, moved into HashMap
    // let is_present = scores.entry(String::from("Blue"));
    // println!("{:?}", is_present);
    scores.entry(String::from("Blue")).or_insert(100);
    scores.entry(String::from("Red")).or_insert(23);
    iterate_over_hash_map(&scores);

    let text = "hello world wonderful world";
    _occurance_of_words(text);
}

fn iterate_over_hash_map(x: &HashMap<String, i32>) {
    for (key, value) in x {
        println!("{}: {}", key, value);
    }
}

fn _occurance_of_words(x: &str) {
    let mut map = HashMap::new();
    for word in x.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
