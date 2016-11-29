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
    let mut x = 1 ;
    println!("{:?}",x +=1);
    println!("===================closure test   end===================");
}