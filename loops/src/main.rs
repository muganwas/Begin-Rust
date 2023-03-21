fn main() {
    let mut num = 0;
    while num < 10 {
        println!("num: {}", num);
        num += 1;
    }
    counter();
}

fn counter() {
    let mut count = 0;
    'count_up: loop {
        println!("counting down: {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining: {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'count_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
}
