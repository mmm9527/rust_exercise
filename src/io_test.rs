use std::fs::File;
use std::io::{self, Stdin, Stdout, Write, Read};

fn read_from_stdin(buf: &mut String) -> io::Result<()> {
    io::stdin().read_line(buf)?;
    Ok(())
}

fn write_from_stdout(buf: &[u8]) -> io::Result<()> {
    io::stdout().write(buf)?;
    Ok(())
}

fn create_file(file_name: &str, buf: &[u8]) -> io::Result<()> {
    let mut f: File = File::create(file_name)?;
    f.write(buf)?;
    Ok(())
}

fn read_file(file_name: &str, mut buf: &mut String) -> io::Result<()> {
    let mut f: File = File::open(file_name)?;
    f.read_to_string(&mut buf)?;
    Ok(())
}

pub fn io_test() {
    println!("===================io_test test begin===================");
    let f = "foo.txt";
    let mut buf = String::new();
    match create_file(f, b"hello world!") {
        Ok(()) => {
            match read_file(f, &mut buf) {
                Ok(()) => { println!("{}", buf); },
                Err(e) => { println!("{}", e); },
            };
        },
        Err(e) => { println!("{}", e); },
    }
    println!("===================io_test test   end===================");
}