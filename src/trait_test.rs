use std::fmt::{Debug, Display};
use std::io::prelude::*;
use std::ops::Add;

trait HasArea {
    fn area(&self) -> f64;
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        ::std::f64::consts::PI * (self.radius * self.radius)
    }
}

fn foo<T: Debug + Clone>(s: T) {
    s.clone();
    println!("{:?}", s);
}

fn foo1<T, K>(t: T, k: K)
    where T: Clone,
          K: Debug + Clone {
    t.clone();
    k.clone();
    println!("{:?}", k);
}

impl HasArea for i32 {
    fn area(&self) -> f64 {
        *self as f64
    }
}

trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String {
        format!("u8:{}", *self)
    }
}

impl Foo for String {
    fn method(&self) -> String {
        format!("String:{}", *self)
    }
}

fn do_something(x: &Foo) {
    println!("{}", x.method());
}

fn add<T: Add<Output=T>>(a: T, b: T) -> T {
    a + b
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T: Add<Output=T>> Add for Point<T> {
    type Output = Point<T>;
    fn add(self, p: Point<T>) -> Point<T> {
        Point {
            x: p.x + self.x,
            y: p.y + self.y,
        }
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
    fn summary(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify<T: Summary + Display>(item: T) {
// pub fn notify(item: impl Summary + Display) {
    println!("Breaking news! {}", item.summarize())
}

/*fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}*/

fn largest_2<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if *item > *largest {
            largest = item;
        }
    }
    largest
}

pub fn trait_test() {
    println!("===================trait test begin  ===================");
    let circle = Circle { x: 1.0, y: 2.0, radius: 3.0 };
    println!("circle={}", circle.area());
    foo(5);
    5.area();
    let x = "hello".to_string();
    do_something(&x);
    let y = 8u8;
    do_something(&y);

    println!("{}", add(100i32, 1i32));
    println!("{}", add(100.11f32, 100.22f32));
    let p1 = Point { x: 1.1f32, y: 1.1f32 };
    let p2 = Point { x: 2.1f32, y: 2.1f32 };
    println!("{:?}", add(p1, p2));

    let p3 = Point { x: 1i32, y: 1i32 };
    let p4 = Point { x: 2i32, y: 2i32 };
    println!("{:?}", add(p3, p4));

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
    hockey team in the NHL."),
    };
    println!("New article available! {}", article.summarize());


    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    println!("===================trait test begin  ===================");
}