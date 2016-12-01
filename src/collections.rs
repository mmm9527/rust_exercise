use std::time;
use std::collections::HashMap;

pub fn collection_test() {
    println!("===================collection test begin===================");
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    println!("vec1={:?}", v1);

    let mut v1 = Vec::<i32>::new();
    v1.push(1);
    println!("vec1={:?}", v1);

    let v2: Vec<_> = (1..5).collect();
    println!("vec2={:?}", v2);

    let a: Vec<i32> = vec![1, 2, 3, 4, 5];
    assert_eq!(2, a[1]);

    assert_eq!(a.get(1), Some(&2) );
    assert_eq!(a.get(5), None );

    let aa = time::SystemTime::now();
    for i in &a {
        println!("{}", i);
    }
    let bb = time::SystemTime::now();
    for i in a.into_iter() {
        println!("{}", i);
    }
    let cc = time::SystemTime::now();
    println!("{:?},{:?}", bb.duration_since(aa).unwrap(), cc.duration_since(bb).unwrap());

    let mut v: Vec<usize> = vec![];
    push_1m(&mut v, 5_000_000);
    let mut v1: Vec<usize> = vec![];
    v1.reserve(5_000_000);
    push_1m(&mut v1, 5_000_000);

    //hashmap
    let mut map = HashMap::new();

    map.insert("key1", "value1");
    map.insert("key2", "value2");
    map.insert("key3", "value3");

    if map.contains_key("key1") {
        println!("find key1");
    }

    assert_eq!(map.remove("key3"), Some("value3") );
    assert_eq!(map.remove("key3"), None );

    for (key, value) in &map {
        println!("key={},value={}", key, value);
    }

    let mut letters = HashMap::new();
    for ch in "a sdfd d ggewoifjjewlfj jewjf".chars() {
        let counter = letters.entry(ch).or_insert(0);
        *counter += 1 ;
    }

//    assert_eq!(2,*(letters.get(&'g').expect("test")));
//    println!("{:?}",letters.get(&'g').expect("test"));

    assert_eq!(letters[&'j'], 5);
    assert_eq!(letters.get(&'y'), None);

    


    println!("===================collection test   end===================");
}

fn push_1m(v: &mut Vec<usize>, total: usize) {
    let b = time::SystemTime::now();
    for i in 1..total {
        v.push(i);
    }
    let e = time::SystemTime::now();
    println!("time spend:{:?}", e.duration_since(b).unwrap());
}