use List::*;


enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    pub fn new() -> List {
        Nil
    }

    pub fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    pub fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => {
                1 + tail.len()
            }
            Nil => 0
        }
    }

    pub fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{},{}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

pub fn test_list(){
    
    println!("{}","-----start list test");
    
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());

    println!("{}","-----end list test");
}
