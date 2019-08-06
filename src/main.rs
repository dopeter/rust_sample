//use std::io;

//fn main2() {
//    println!("Hello, world!");
//
//    println!("Guess the number!");
//
//    println!("Please input your guess.");
//
//    let mut guess=String::new();
//
//    io::stdin().read_line(&mut guess)
//        .expect("Failed to read line");
//
//    println!("You guessed:{}",guess)
//}

mod algorithms;


use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::io;
use std::process::exit;
use crate::algorithms::data_structure::stack::Stack;


fn main() {
    testStack();
    let filename = "./1.txt";

    match read_username_from_file() {
        Ok(f) => {
            println!("ok very goodsdfsdf")
        }
        Err(ex) => {
            println!("{}", ex.description());
            exit(1);
        }
    }

    match test_open_file(filename) {
        Ok(f) => {
            println!("ok very good")
        }
        Err(ex) => {
            println!("{}", ex.description())
        }
    }


//    let mut f = File::open(filename).unwrap();

    let i = 0;
    let p1 = &i;
    let p2 = &i;
    println!("{}{}{}", i, *p1, p2);

    let mut v = vec![1, 2, 3];
    v.push(4);

    print!("{},{},", v[0], v[1]);

    v.push(4);

    println!("{}", v[3]);
}

fn test_open_file(filename: &str) -> Result<i32, Box<Error>> {
    let mut file = File::open(filename)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut sum = 0;
    for c in contents.lines() {
        let n: i32 = c.parse::<i32>()?;
        sum += n;
    }
    Ok(sum)
}

fn read_username_from_file() -> Result<String, Box<Error>> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn testStack() {
    #[derive(PartialEq, Eq, Debug)]
    struct TestStruct {
        a: i32,
    }

    let a = TestStruct { a: 5 };
    let b = TestStruct { a: 9 };

    let mut s = Stack::<&TestStruct>::new();
    assert_eq!(s.pop(), None);

    s.push(&a);
    s.push(&b);

    println!("{:?}", s);

    assert_eq!(s.pop(), Some(&b));
    assert_eq!(s.pop(), Some(&a));
    assert_eq!(s.pop(), None);
}
