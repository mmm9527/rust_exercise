#[derive(Debug)]
struct Foo {
    a: i32,
    b: bool,
}

impl Copy for Foo {}

impl Clone for Foo {
    fn clone(&self) -> Self {
        Foo { a: self.a, b: self.b }
    }
}

pub fn ownership_test() {
    println!("===================ownership test begin===================");
    let f = Foo { a: 1, b: false };
    let c = f.clone();
    println!("Foo's clone={:?}", c);

    let mut x: String = String::from("hello");
    {
        let mut some_closure = |c: char| x.push(c);
        let y = some_closure('d');
    }
    println!("x={:?}", x);

    println!("===================ownership test   end===================");
}