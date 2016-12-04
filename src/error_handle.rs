use std::num::ParseIntError;

fn find_file_ex(name: &str) -> Option<&str> {
    match name.find('.') {
        None => None,
        Some(i) => Some(&name[i + 1..])
    }
}

fn find_file_ex2(name: &str) -> Option<&str> {
    name.find('.').map(|i| &name[i + 1..])
}

fn double_number(number: &str) -> i32 {
    2 * number.parse::<i32>().unwrap()
}

fn double_number2(number: &str) -> Result<i32, ParseIntError> {
    number.parse::<i32>().map(|n| n * 2)
}


pub fn error_handle_test() {
    println!("===================error_handle test begin===================");

    match find_file_ex2("foo.rs") {
        None => println!("no extension"),
        Some(ext) => assert_eq!(ext, "rs" ),
    }

    let n: i32 = double_number("10");
    assert_eq!(n, 20);

    match double_number2("10a") {
        Ok(n) => assert_eq!(n, 20),
        Err(err) => println!("Error : {:?}", err),
    }


    println!("===================error_handle test   end===================");
}