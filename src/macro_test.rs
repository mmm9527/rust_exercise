macro_rules! create_function {
    ($func_name:ident) => (
        fn $func_name() {
              println!("function {:?} is called", stringify!($func_name))
        }
    );
}

fn foo1(x: i32) -> i32 {
    x * x
}

macro_rules! foo1 {
    ($x: ident) => {println!("{:?}",$x);};
}

macro_rules! vector {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(temp_vec.push($x);)*
            temp_vec
        }
    };
}

macro_rules! find_min {
    ($x:expr) => ($x);
    ($x:expr,$($y:expr),+) => (
        $crate::std::cmp::min($x,find_min!($($y),+))
    );
}

macro_rules! foo {
    () => (
        let x = 3;
    );
}

macro_rules! bar {
    ($x:ident) => (let $x = 3 ;);
}

macro_rules! foo_fn {
    () => ( fn x(){});
}

pub fn macro_test() {
    println!("===================macro_test test begin===================");

    create_function!(foo);
    foo();
    let a = 5;

    foo1!(a);
    println!("{}", foo1(a));

    let a = vector![1,2,4,8];
    println!("{:?}", a);

    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2 , 2u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 4u32));

    //    foo!();
    //    println!("{}", x);
    bar!(a);
    println!("{}", a);

    foo_fn!();
    x();

    println!("===================macro_test test   end===================");
}