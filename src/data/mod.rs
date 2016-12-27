
use std::mem;
use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use std::fmt::Debug;
use rand::distributions::{IndependentSample, Range};
use rand;

pub fn data_test() {
    println!("===================basic data test begin===================");
    let x = 5;
    let point = &x as *const i32;
    let raw_point = unsafe { *point };
    println!("x 's raw point={:?},reference={}", point, raw_point);

    let char = 'c';
    println!("char's size={}", mem::size_of::<char>());


    let x: f64 = 1.23234324e+2;
    println!("f64 x={}", x);
    println!("f64 as i32 ={}", x as i32);
    println!("f64 abs_sub={}", (x - 125f64).abs());

    let s_char = b'h';
    println!("b'h' size is {}", mem::size_of_val(&s_char));

    let str_char = b"hello";
    println!(r#"b"hello" 's size {} "#, mem::size_of_val(str_char));

    let t1: (i32, i32) = (32, 33);
    let t2: (i32, i32) = (32, 33);
    assert_eq!(t1, t2);

    if t1 == t2 {
        println!("tuple t1 equal t2");
    };
    // array 、 Vec
    let mut array = [0; 3];
    println!("array={:?}", array);
    println!("array's size={}", mem::size_of_val(&array));
    array[1] = 1;
    array[2] = 2;
    println!("&array[1..]={:?}", &array[1..]);
    assert_eq!([1, 2], &array[1..]);
    for i in &array {
        println!("{}", i);
    };

    let vec1: Vec<i32> = Vec::new();
    println!("vec1's size={}", mem::size_of_val(&vec1));

    let v2: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("v2's size={}", mem::size_of_val(&v2));
    ;

    let v3: Vec<i32> = vec![0; 10];
    println!("v3={:?}", v3);

    //str、 string
    // 从 `&str` 类型转化成 `String` 类型
    let s1: String = "hello mzy".to_string();
    let s2: String = String::from("hello mzy");
    //从 String 转换成 str
    let s3: &str = &s1;
    println!("s1 convert to str = {}", s3);
    println!("str's size={}", mem::size_of_val(s3));

    //struct
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32
    }
    let mut p1 = Point { x: 0, y: 2 };
    println!("struct p1={:?}", p1);

    struct Color(u8, u8, u8);
    println!("struct Color's size={}", mem::size_of::<Color>());
    let c1: Color = Color(1, 2, 3);
    println!("struct c1's size={}", mem::size_of_val(&c1));
    #[derive(Debug)]
    struct Digit(i32);
    let v: Vec<i32> = vec![0, 1, 2];
    let d: Vec<Digit> = v.into_iter().map(Digit).collect();
    println!("d={:?}", d);
    struct Inches(i32);
    let inches = Inches(1);
    let Inches(i1) = inches;
    println!("inches={}", i1);
    // unit-like structs
    struct EmptyStruct;
    let empty = EmptyStruct;

    #[derive(Debug, Default)]
    struct Point3d {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin: Point3d = Point3d::default();
    let point = Point3d { y: 1, ..origin };
    let Point3d { x: x0, y: y0, .. } = point;

    println!("point={:?}", point);
    println!("Point3d={},{}", x0, y0);

    println!("size of '()' {} ", mem::size_of::<()>());
    println!("size of 'struct EmptyStruct;' {} ", mem::size_of::<EmptyStruct>());

    //枚举
    #[derive(Debug)]
    enum Message {
        Quit,
        ChangeColor(i32, i32, i32),
        Move {
            x: i32,
            y: i32
        },
        Write(String),
    }
    let eu: Message = Message::ChangeColor(1, 2, 3);
    println!("enum message ChangeColor={:?}", eu);
    let between = Range::new(1i32, 1000i32);
    let mut rng = rand::thread_rng();
    let x = between.ind_sample(&mut rng);
    match x {
        0 | 1 => println!("0 or 1"),
        2 ... 50 => println!("2 ~ 50"),
        50 ... 100 => println!("50 ~ 100"),
        n @ 100 ... 1000 => println!("100 ~ 1000 = {}", n),
        _ => println!("other "),
    };

    match point {
        Point3d { y, .. } => println!("point3d'y={}", y),
    };

    println!("struct 's y={}", origin.x);

    let x = 5;
    let mut y = 10;

    match x {
        ref r => println!("got x's reference {}", r),
    };
    match y {
        ref mut y => println!("get y's reference {}", y),
    };

    let pair = (12, 1);
    match pair {
        (0, x) => println!("pair(0,{})", x),
        (y, 0) => println!("pair({},0)", y),
        (x, y) => println!("pair({},{})", x, y)
    };

    let mm: Message = Message::Write("123132".to_string());
    match mm {
        Message::ChangeColor(..) => println!("Message ChangeColor"),
        Message::Move { .. } => println!("message move"),
        Message::Quit => println!("message quit"),
        Message::Write(a) => println!("message write {}", a)
    };

    let num = Some(10);
    if let Some(i) = num {
        println!("num = {}", i);
    } else {
        println!("num not match !");
    };

    let mut option = Some(1);
    while let Some(a) = option {
        if a > 9 {
            println!("Greater than 9, quit!");
            option = None;
        } else {
            println!("a is {}", a);
            option = Some(a + 1);
        };
    };

    //    let x: i32 = diverges();
    //    let y: String = diverges();


    println!("===================basic data test end  ===================");
}

