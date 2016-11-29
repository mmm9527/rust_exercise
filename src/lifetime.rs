fn foo(x: &str) -> &str {
    x
}

fn foo1<'a>(x: &'a str) -> &'a str {
    x
}

fn foo2<'a>(x: &'a str, y: &'a str) -> &'a str {
    if true {
        x
    } else {
        y
    }
}

fn foo3<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
    if true {
        x
    } else {
        y
    }
}

struct Person<'a> {
    a: &'a u8,
}

impl<'a, 'b: 'a> Person<'a> {
    fn print_age(&self) {
        println!("age={}", self.a);
    }
    fn get_age(&self) -> &u8 {
        self.a
    }

    fn get_max_age(&'a self, p: &'a Person) -> &'a u8 {
        if self.a > p.a {
            self.a
        } else {
            p.a
        }
    }
}

pub fn lifetime_test() {
    println!("===================lifetime test begin===================");
    println!("{}", foo("test"));
    println!("{}", foo1("test"));
    println!("{}", foo2("test", "hello"));
    println!("{}", foo3("test", "hello"));

    let x = 32u8;
    let p = Person {
        a: &x,
    };
    println!("x={}", x);

    p.print_age();
    p.get_age();


    println!("===================lifetime test   end===================");
}