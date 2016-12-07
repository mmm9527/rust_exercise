use std::rc::{Rc, Weak};
use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc::channel;
use std::thread;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;

struct Owner {
    name: String
}

struct Gadget {
    id: i32,
    owner: Rc<Owner>
}

#[derive(Debug)]
struct Teacher {
    name: String,
    students: RefCell<Vec<Weak<Student>>>
}

#[derive(Debug)]
struct Student {
    id: i32,
    teacher: Rc<Teacher>
}

pub fn smart_pointer_test() {
    println!("===================smart_pointer test begin===================");

    //Rc

    let five = Rc::new(5);
    let five1 = five.clone();
    let five2 = five.clone();
    println!("src={},rc1={},rc2={}", five, five1, five2);

    let five3 = &five ;
    assert!(Rc::ptr_eq(&five,five3));

    let five = Rc::new(5);
    let weak_five = Rc::downgrade(&five);
    let strong_five = weak_five.upgrade();

    println!("src={},weak={:?},strong={:?}", five, weak_five, strong_five);

    //Arc
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
    const N: usize = 10;

    let data = Arc::new(Mutex::new(0));
    let (tx, rx) = channel();

    for i in 0..10 {
        let (data, tx) = (data.clone(), tx.clone());

        thread::Builder::new().name("thread name :".to_string() + &i.to_string()).spawn(move || {
            let mut data = data.lock().unwrap();
            println!("thread:{} sending...", thread::current().name().unwrap());
            *data += 1;
            if *data == N {
                tx.send(()).unwrap();
            }
        }).unwrap();
    }

    rx.recv().unwrap();

    //RwLock
    let lock = RwLock::new(5);
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    }
    {
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);
    }

    //Cell, RefCell

    let c = Cell::new(5);
    let five = c.get();
    println!("cell {:?},{}", c, five);

    c.set(10);
    println!("cell updated {:?},{}", c, five);

    let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
    shared_map.borrow_mut().insert("africa", 92388);
    shared_map.borrow_mut().insert("kyoto", 11837);
    shared_map.borrow_mut().insert("piccadilly", 11826);
    shared_map.borrow_mut().insert("marbles", 38);

    println!("map={:?}", shared_map.borrow());

    let result = thread::spawn(|| {
        let r = RefCell::new(5);
        let a = r.borrow_mut();
        //        let b = r.borrow();
    }).join();

    //    assert_eq!(true, result.is_err());

    let student_teacher: Rc<Teacher> = Rc::new(Teacher {
        name: "teacher1".to_string(),
        students: RefCell::new(Vec::new())
    });

    let student1 = Rc::new(Student { id: 1, teacher: student_teacher.clone() });
    let student2 = Rc::new(Student { id: 2, teacher: student_teacher.clone() });

    student_teacher.students.borrow_mut().push(Rc::downgrade(&student1));
    student_teacher.students.borrow_mut().push(Rc::downgrade(&student2));

    for student_weak in student_teacher.students.borrow().iter() {
        let student = student_weak.upgrade().unwrap();
        println!("student={:?}", student);
    }


    println!("===================smart_pointer test   end===================");
}