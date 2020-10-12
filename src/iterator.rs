use std::mem;
use std::collections::HashMap;

struct Person{
    name: String,
    age: u64,
}

pub fn iterator_test() {
    println!("===================iterator test begin===================");
    let v = vec![1, 2, 3, 4, 5];

    {
        let result = match IntoIterator::into_iter(v) {
            mut iter => loop {
                match iter.next() {
                    Some(x) => { println!("{}", x); },
                    None => break,
                }
            },
        };
        result
    }

    let v = (1..5).collect::<Vec<_>>();
    let v: Vec<_> = (1..5).collect();

    let m = (1..20).fold(1u64, |mul, x| mul * x);
    println!("m={}", m);

    let o: Vec<_> = (1..20).map(|x| x + 1).filter(|x| x & 1 == 0).collect();
    println!("o={:?}", o);

    let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let v_take = v.iter().cloned().take(2).collect::<Vec<i32>>();
    assert_eq!(v_take, vec![1, 2]);
    let v_skip = v.iter().cloned().skip(2).collect::<Vec<i32>>();
    assert_eq!(v_skip, vec![3, 4, 5, 6, 7, 8, 9]);

    let n = vec!["m1", "m2", "m3"];
    let n1 = vec![1, 2, 3];

    let s: <_, _> = n.iter().zip(n1.iter()).collect();
    println!("s={:?}", s);

    let v = vec![1u64, 2, 3, 4, 5, 6];

    let val = v.iter().enumerate().filter(|&(idx, _)| idx & 1 == 0).map(|(idx, val)| val).fold(0u64, |sum, acm| sum + acm);

    println!("val={}", val);

    let sxh = (100..).map(|x| {
        (x, format!("{}", x))
    }).filter(|&(a, ref b)| {
        let size = b.len();
        let bool = b.chars().map(|val| {
            (val as u64 - '0' as u64).pow((size as u32))
        }).fold(0u64, |sum, acm| sum + acm);
        a == bool
    }).map(|(l, _)| l).take(10).collect::<Vec<u64>>();
    println!("sxh={:?}", sxh);

    let mut vec1: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    /*let ref mut r1 = vec1;
    println!("{:?}", r1);
    println!("{:?}", mem::size_of_val(r1));*/

    let r1 = &mut vec1;

    println!("{:?}", r1);
    println!("{:?},{:?}", mem::size_of_val(r1), mem::size_of::<Person>());

    let a1 = [0, 1, 2];
    let mut iter = a1.into_iter().filter(|&&x| x > 1);
    println!("{:?}",iter.next());


    println!("===================iterator test   end===================");
}