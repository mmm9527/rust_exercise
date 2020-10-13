use std::fmt::Display;

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

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
        // announcement
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
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


    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is: {}", result);
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let important_excerpt = ImportantExcerpt { part: first_sentence };
    println!("{:?}", important_excerpt);

    //所有的字符串字面值都拥有 'static 生命周期
    let s: &'static str = "I have a static lifetime";
    println!("===================lifetime test   end===================");
}