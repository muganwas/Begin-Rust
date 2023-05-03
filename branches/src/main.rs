fn main() {
    let num = 3;
    if num > 0 {
        println!("Number is greater than zero");
    } else {
        println!("Number is less than zero");
    }
    error_inducer(0);
}

fn error_inducer(x: i32) {
    let word = if x != 0 {
        "Number is not zero"
    } else {
        "Number is zero"
    };
    println!("{word}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_inducer() {
        error_inducer(0);
    }
}
