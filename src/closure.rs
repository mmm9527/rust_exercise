pub fn closure_test() {
    println!("===================closure test begin===================");
    let plus_one = |x| x + 1;
    assert_eq!(2, plus_one(1));
    let plus_two = |x| {
        let mut result: i32 = x;

        result += 1;
        result += 1;

        result
    };
    assert_eq!(4, plus_two(2));
    let mut x = 1;
    println!("{:?}", x += 1);

    let mut x: Vec<i32> = vec![1, 2, 3, 4, 5];
    let x1 = || &x;
    println!("{:?}", x);
    let y = x1();
    println!("x={:?},y={:?}", x, y);

    let result = call_with_one(|x| x + 2);

    assert_eq!(result, 3);

    let ret = call_with_one1(&|x| x + 2);

    assert_eq!(result, 3);

    let answer = call_with_one1(&add_one);

    assert_eq!(2, answer);

    println!("===================closure test   end===================");
}
//静态分发
fn call_with_one<F>(some_closure: F) -> i32
    where F: Fn(i32) -> i32 {
    some_closure(1)
}

fn call_with_one1(some_closure: &Fn(i32) -> i32) -> i32 {
    some_closure(1)
}

fn add_one(i: i32) -> i32 {
    i + 1
}
