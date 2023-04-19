fn main() {
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let largest = _largest_i32(&number_list);
    println!("The largest number is {}", largest);
    let char_list = vec!['y', 'm', 'a', 'q', 'z'];
    let largest = _largest_charactor(&char_list);
    println!("The largest charactor is {}", largest);
}

fn _largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn _largest_charactor(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
