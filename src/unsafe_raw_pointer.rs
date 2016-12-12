unsafe fn foo() {
    println!("this is unsafe method !");
}

unsafe trait Scary {}

unsafe impl Scary for i32 {}


pub fn unsafe_raw_pointer() {
    println!("===================unsafe_raw_pointer test begin===================");

    let x = 5i32;
    let raw_p = &x as *const i32;
    println!("raw pointer:{:?}", raw_p);

    let points_at = unsafe { *raw_p };
    println!("raw points at {}", points_at);

    println!("raw pointer:{:p}", &x);

    static mut N: i32 = 5;
    unsafe {
        N += 1;
        println!("N: {}", N);
    }
    unsafe {
        foo();
    }

    //裸指针
    let a = 1;
    let b = &a as *const i32;
    println!("{:?}", b);
    let mut x = 2;
    let y = &mut x as *mut i32;
    println!("y = {:?}", y);
    println!("y = {:#?}", y);

    unsafe {
        *y = 5;
    }
    println!("{}", x);

    //Box::into_raw
    let a: Box<i32> = Box::new(10);
    // 我们需要先解引用a，再隐式把 & 转换成 *
    let b: *const i32 = &*a;
    // 使用 into_raw 方法
    let c: *const i32 = Box::into_raw(a);

    println!("b = {:#?} , c = {:#?}", b, c);

    // 显式
    let a = 1;
    let b: *const i32 = &a as *const i32; //或者let b = &a as *const i32；
    // 隐式
    let c: *const i32 = &a;
    unsafe {
        println!("{}", *c);
    }

    println!("===================unsafe_raw_pointer test   end===================");
}