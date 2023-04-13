use std::{
    fs::{self, File},
    io::{Error, ErrorKind, Read},
};

fn main() {
    let _f_result = File::open("hello.txt");
    //_check_for_file(_f_result);
    //_f_result.expect("Failed to open hello.txt");
    let _username = _fs_read_to_string();
    match _username {
        Ok(username) => println!("Username: {}", username),
        Err(e) => panic!("{}", e),
    }
    //println!("Username: {:?}", _username); // will print a Result with the username or the error
}

fn _check_for_file(f: Result<File, Error>) {
    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => {
                panic!("There was a problem opening the file: {:?}", other_error)
            }
        },
    };
}

fn _read_username_from_file() -> Result<String, Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn _unary_postfix() -> Result<String, Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn _fs_read_to_string() -> Result<String, Error> {
    fs::read_to_string("hello.txt")
}

fn _read_last_xter(x: &str) -> Option<char> {
    x.lines().next()?.chars().last()
}
