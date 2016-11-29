pub fn borrowing_test() {
    println!("===================borrow test begin===================");

    let x: Vec<i32> = vec![1, 2, 3];
    let y = &x;
    println!("x={:?},y={:?}", x, y);

    let a = &x as *const Vec<i32>;
    let b = y as *const Vec<i32>;
    println!("x pointer={:?}, y pointer={:?}", a, b);

    let mut x: i32 = 100;
    {
        let mut y = &mut x;
        *y += 2;
    }
    println!("x={}", x);

    println!("===================borrow test   end===================");
}