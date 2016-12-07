use std::borrow::{Borrow, Cow};
use std::rc::Rc;

fn is_hello<T: Into<Vec<u8>>>(s: T) {
    let bytes = b"hello".to_vec();
    assert_eq!(bytes, s.into());
}

fn is_str<T: AsRef<str>>(s: T) {
    assert_eq!("hello", s.as_ref());
}

#[derive(Debug)]
struct Person {
    name: String,
}

impl Person {
    fn new<S: Into<String>>(s: S) -> Person {
        Person {
            name: s.into(),
        }
    }
}

fn check<T: Borrow<str>>(s: T) {
    assert_eq!("hello", s.borrow());
}

fn foo(s: &str) {
    println!("str={}", s);
}

fn foo1(s: &[i32]) {
    println!("[i32]={:?}", s);
}

#[derive(Debug)]
struct Foo;

impl Foo {
    fn foo(&self) { println!("Foo"); }
}

fn abs_all(input: &mut Cow<[i32]>) {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            println!("input{:?}", input);
            input.to_mut()[i] = -v;
        }
    }
}

pub fn common_trait_test() {
    println!("===================common_trait test begin===================");
    //Into/From 及其在 String 和 &str 互转上的应用

    let s = "hello".to_string();
    let other = String::from("hello");
    assert_eq!(s, other);

    is_hello(s);

    let p1 = Person::new("hello");
    let p2 = Person::new("hello".to_string());
    println!("{:?},{:?}", p1, p2);

    let s = "hello";
    is_str(s);
    let s = "hello".to_string();
    is_str(s);


    let s = "hello";
    check(s);
    let s = "hello".to_string();
    check(s);


    // String implements Deref<Target=str>
    let owned = "hello".to_string();

    foo(&owned);

    let counted = Rc::new(owned);
    foo(&counted);

    // Vec<T> implements Deref<Target=[T]>
    let owned = vec![1, 2, 3];

    foo1(&owned);


    let ff = &Foo;
    println!("{:?}", ff as *const Foo);

    let fff = &ff;
    println!("{:?}", *fff as *const Foo);

    fff.foo();
    (&fff).foo();
    (&&fff).foo();
    (&&&&&&&&fff).foo();

    //Cow
    let mut cow: Cow<[_]> = Cow::Owned(vec![1, 2, 3]);
    {
        let hello = cow.to_mut();
        assert_eq!(hello, &[1, 2, 3]);
    }
    let hello = cow.into_owned();

    assert_eq!(vec![1, 2, 3], hello);

    abs_all(&mut Cow::Owned(vec![1, -2, 3, -12, -43]));

    let ss = "hello world";
    let ds = remove_space(ss);
    println!("result={}", ds);

    println!("===================common_trait test   end===================");
}

fn remove_space<'a>(input: &'a str) -> Cow<'a, str> {
    if input.contains(' ') {
        let mut buff = String::with_capacity(input.len());
        for c in input.chars() {
            if c != ' ' {
                buff.push(c);
            }
        }

        return Cow::Owned(buff);
    }
    Cow::Borrowed(input)
}