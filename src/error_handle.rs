use std::num::ParseIntError;
use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::error::Error;

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

fn double_args(mut argv: env::Args) -> Result<i32, String> {
    argv.nth(1)
        .ok_or("please give at least one argument".to_owned())
        .and_then(|arg| arg.parse::<i32>().map_err(|err| err.to_string()))
        .map(|n| n * 2)
}

fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    File::open(file_path)
        .map_err(|err| err.to_string())
        .and_then(|mut file| {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|err| err.to_string())
                .map(|_| contents)
        })
        .and_then(|contents| {
            contents.trim().parse::<i32>()
                .map_err(|err| err.to_string())
        })
        .map(|n| n * 2)
}

fn file_double2<P: AsRef<Path>>(file_path: P) -> Result<i32, Box<Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let n = contents.trim().parse::<i32>()?;
    Ok(2 * n)
}

#[derive(Debug)]
enum CliError {
    Io(io::Error),
    Parse(ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}

impl From<ParseIntError> for CliError {
    fn from(err: ParseIntError) -> CliError {
        CliError::Parse(err)
    }
}

fn file_double_verbose<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError> {
    let mut file = File::open(file_path).map_err(CliError::Io)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(CliError::Io)?;
    let n = contents.trim().parse::<i32>().map_err(CliError::Parse)?;
    Ok(2 * n)
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

    match double_args(env::args()) {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error:{}", err),
    }

    match file_double("foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error:{}", err),
    }

    match file_double2("foobar1") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error:{}", err),
    }
    match file_double_verbose("foobar1") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error:{:?}", err),
    }

    println!("===================error_handle test   end===================");
}