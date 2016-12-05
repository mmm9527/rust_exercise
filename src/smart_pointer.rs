use std::rc::{Rc, Weak};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread;

struct Owner {
    name: String
}

struct Gadget {
    id: i32,
    owner: Rc<Owner>
}

const N : usize = 10 ;

pub fn smart_pointer_test() {
    println!("===================smart_pointer test begin===================");

    let five = Rc::new(5);
    let five1 = five.clone();
    let five2 = five.clone();
    println!("src={},rc1={},rc2={}", five, five1, five2);

    let five = Rc::new(5);
    let weak_five = Rc::downgrade(&five);
    let strong_five = weak_five.upgrade();

    println!("src={},weak={:?},strong={:?}", five, weak_five, strong_five);

    let numbers: Vec<_> = (0..5u32).collect();

    let shared_numbers = Arc::new(numbers);
    for i in 0..10 {
        let child_numbers = shared_numbers.clone();
        thread::Builder::new().name("thread name :".to_string() + &i.to_string()).spawn(move || {
            let local_numbers = &child_numbers[..];
            println!("thread:{:?},numbers={:?}", thread::current().name().unwrap(), local_numbers);
        }).unwrap();
    }

    let gadget_owner: Rc<Owner> = Rc::new(
        Owner { name: String::from("Gadget Man") }
    );
    let gadget1 = Gadget { id: 1, owner: gadget_owner.clone() };
    let gadget2 = Gadget { id: 2, owner: gadget_owner.clone() };

    drop(gadget_owner);

    println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
    println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);

    //Mutex


    println!("===================smart_pointer test   end===================");
}