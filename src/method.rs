extern crate rand;

use std::mem;
use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use std::fmt::Debug;
use self::rand::distributions::{IndependentSample, Range};

pub fn method_test() {
    println!("===================method test begin  ===================");

    //函数

    let x = 5;
    let x_plus = |y: i32| x + y;

    println!("x({}) + y ={}", x, x_plus(10));


    let f0: fn(i32) -> i32 = add_one;
    let f = add_one(2) * 2;
    let f1 = apply(add_one, 10);
    let f2 = apply(add_one, 10);

    println!("f0={:?},f={},f1={},f2={}", f0, f, f1, f2);

    let box_fn = factory(1);
    let b0 = box_fn(2) * 2;
    let b1 = (*box_fn)(2) * 2;
    let b2 = (&box_fn)(2) * 2;
    println!("{},{},{}", b0, b1, b2);

    let add_num = &(*box_fn);
    let trans: &Fn(i32) -> i32 = add_num;
    let z0 = add_num(2i32) * 2;
    let z1 = apply(add_num, 2);
    let z2 = apply(trans, 2);
    println!("{}, {}, {}", z0, z1, z2);

    //方法

    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }
    impl Circle {
        fn new(x: f64, y: f64, radius: f64) -> Circle {
            Circle { x: x, y: y, radius: radius }
        }
        fn area(&self) -> f64 {
            ::std::f64::consts::PI * (self.radius * self.radius)
        }
    }
    println!("PI={}", ::std::f64::consts::PI);

    let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
    println!("Circle={}", c.area());
    println!("Circle={}", Circle::new(0.0, 0.0, 2.0).area());

    //trait

    trait HasArea {
        fn area(&self) -> f64;
    }
    impl HasArea for Circle {
        fn area(&self) -> f64 {
            ::std::f64::consts::PI * (self.radius * self.radius)
        }
    }

    struct Square {
        x: f64,
        y: f64,
        side: f64,
    }
    impl HasArea for Square {
        fn area(&self) -> f64 {
            self.side * self.side
        }
    }

    fn print_area<T: HasArea>(shape: T) {
        println!("This shape has an area of {}", shape.area());
    }

    //泛型和多态
    trait Graph {
        type N;
        type E;
        fn has_edge(&self, &Self::N, &Self::N) -> bool;
        fn edges(&self, &Self::N) -> Vec<Self::E>;
    }
    struct Node;
    struct Edge;
    #[derive(Debug)]
    struct SimpleGraph;

    impl Graph for SimpleGraph {
        type N = Node;
        type E = Edge;

        fn has_edge(&self, n1: &Node, n2: &Node) -> bool {
            unimplemented!()
        }

        fn edges(&self, n1: &Node) -> Vec<Edge> {
            unimplemented!()
        }
    }

    let graph: SimpleGraph = SimpleGraph;
    let object: Box<Graph<N = Node, E = Edge>> = Box::new(graph);
    /*
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("WTF");
    println!("you typed :{}", input);
    */
    //文件输入输出
    let path = Path::new("hello.txt");
    let display = path.display();
    let mut file: File = File::open(&path).unwrap() ; //if !path.exists() { File::create(&path).unwrap() } else { File::open(&path).unwrap() };
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    println!("file 's context={}", s);
    file = OpenOptions::new().append(false).write(true).open(&path).unwrap();
    file.write_all("不知道".as_bytes()).unwrap();

    //条件分支

    //if let
    let x = Some(5);
    if let Some(y) = x {
        println!("y={}", y);
    }
    let z = if let Some(y) = x {
        y
    } else {
        0
    };
    println!("z={}", z);
    let mut v1 = vec![1, 2, 3, 4, 5, 6];
    let v2 = vec![0; 10];
    println!("v1={}", v1[0]);

    for i in &mut v1 {
        *i = *i + 1;
        print!("{}", i);
    }
    println!("");
    for i in &v1 {
        print!("{}", i);
    }
    println!("");

    struct RefBody<'a, R> {
        loc: &'a i32,
        r: R
    }
    let x = 5;
    let y = &x;
    let rfb = RefBody { loc: y, r: 12 };
    println!("y={}", y);

    let mm = vec![229, 150, 181];
    let mn = String::from_utf8(mm).unwrap();
    println!("mn={}", mn);

    let x: String = "哎哟我去".to_string();
    for i in x.as_bytes() {
        print!("{} ", i);
    }

    println!("");

    for i in x.chars() {
        print!("{}", i);
    }
    println!("");

    println!("{}", x.chars().nth(2).unwrap());

    println!("(0)={}", (0));

    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut b = Vec::<i32>::new();
    for i in &a {
        b.push(get_func(*i)(*i));
    }
    println!("{:?}", b);

    let f = get_fun();
    println!("cf={}", f(1));

    let tuple: (u32, String) = (5, String::from("test"));

    let (x, _) = tuple;
    println!("{:?}", tuple);

    let mut x = 5;
    match x {
        ref mut mr => println!("mut ref {}", mr),
    }
    let ref mut mrx = &x;
    println!("ref mut x ={}", mrx);
    println!("x={}", x);

    let test = 5u32;
    match test {
        n @ 1 ... 5 | n @ 5 ... 10 => println!("n={}", n),
        _ => println!("else")
    }
    #[derive(Debug)]
    struct Person {
        name: Option<String>,
    }
    let name = "mzy".to_string();
    let x: Option<Person> = Some(Person { name: Some(name) });
    match x {
        Some(Person { name: ref a @ Some(_), .. }) => println!("name={:?}", a),
        _ => {}
    }
    println!("===================method test end  ===================");
}

fn get_fun() -> Box<Fn(i32) -> i32> {
    let a = 1i32;
    Box::new(move |x| x + a)
}

fn get_func(n: i32) -> fn(i32) -> i32 {
    fn inc(n: i32) -> i32 {
        n + 1
    }
    fn dec(n: i32) -> i32 {
        n - 1
    }
    if n % 2 == 0 {
        inc
    } else {
        dec
    }
}

fn process(n: i32, func: fn(i32) -> i32) -> i32 {
    func(n)
}

fn process1<F>(n: i32, func: F) -> i32
    where F: Fn(i32) -> i32 {
    func(n)
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn apply<F>(f: F, y: i32) -> i32
    where F: Fn(i32) -> i32 {
    f(y) * y
}

fn factory(x: i32) -> Box<Fn(i32) -> i32> {
    Box::new(move |y| x + y)
}

fn diverges() -> ! {
    panic!("This function never returns!")
}