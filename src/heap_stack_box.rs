
fn foo(x: &str) -> String {
    "hello".to_string() + x
}

//#![feature(box_syntax, box_patterns)]
pub fn heap_stack_box_test() {
    println!("===================heap_stack_box test begin===================");

    let b = foo("a");
    println!("{}", b);

    let boxed = Some(box 5);
    match boxed {
        Some(box unboxed) => println!("Some {}", unboxed),
        None => println!("None"),
    }

    println!("===================heap_stack_box test   end===================");
}