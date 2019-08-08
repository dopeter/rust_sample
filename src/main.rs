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
use algorithms::data_structure::stack::Stack;
use algorithms::data_structure::queue::Queue;
use algorithms::data_structure::binary_search::*;
use std::borrow::BorrowMut;
use algorithms::data_structure::priority_queue;
use algorithms::data_structure::priority_queue::*;
use algorithms::data_structure::list::*;
use algorithms::data_structure::simply_graph::*;


fn main() {
    test_stack();

    test_queue();

    test_binary_search();

    priority_queue_test_aggregate();

    test_list();

    test_simply_graph();

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

fn test_stack() {
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

fn test_queue(){

    let mut q=Queue::new();

    q.push(1);
    q.push(2);

    println!("{:?}",q);

    q.pop();

    println!("{:?}",q);

    let item=q.pop();

    println!("{:?}",item);

    q.pop();
}

fn test_binary_search(){

    let mut root=Node::<i32,i32>::new(3,4);

    root.insert(2,3);
    root.insert(4,6);
    root.insert(5,5);
    root.insert(6,6);
    root.insert(1,8);

    if let Some(ref right)=root.right{
        assert_eq!(right.value,6);

        if let Some(ref right)=right.right{
            assert_eq!(right.value,5);
        }
    }

    println!("Pre Order traversal");
    root.pre_order();
    println!("In Order traversal");
    root.in_order();
    println!("Pos Order traversal");
    root.pos_order();

}