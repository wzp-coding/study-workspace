use std::io::Read;
use std::{fs, io};
use std::{fs::File, io::ErrorKind};

fn handle_error() {
    let f = File::open("hello.txt");
    // 第一种（比较冗余）
    let f1 = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // 第二种
    let f2 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    
}

fn read_username_from_file() -> Result<String, io::Error> {
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

fn read_username_from_file1() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn spread_error() {
    let res = read_username_from_file();
    let res1 = read_username_from_file1();
    let res2 = read_username_from_file2();
    let res3 = read_username_from_file3();
    println!("{:?}", res);
    println!("{:?}", res1);
    println!("{:?}", res2);
    println!("{:?}", res3);
}

fn main() {
    handle_error();
    spread_error();
}
