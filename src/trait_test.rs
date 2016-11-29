use std::io::prelude::*;
use std::fmt::Debug;
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

fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Point<T>;
    fn add(self, p: Point<T>) -> Point<T> {
        Point {
            x: p.x + self.x,
            y: p.y + self.y,
        }
    }
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
    let p1 = Point{x: 1.1f32, y: 1.1f32};
    let p2 = Point{x: 2.1f32, y: 2.1f32};
    println!("{:?}", add(p1, p2));

    let p3 = Point{x: 1i32, y: 1i32};
    let p4 = Point{x: 2i32, y: 2i32};
    println!("{:?}", add(p3, p4));

    println!("===================trait test begin  ===================");
}