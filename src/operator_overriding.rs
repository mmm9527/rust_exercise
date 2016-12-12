use std::ops::Add;

#[derive(Debug)]
struct Complex {
    a: f64,
    b: f64,
}

impl Add for Complex {
    type Output = Complex;
    fn add(self, other: Complex) -> Complex {
        Complex { a: self.a + other.a, b: self.b + other.b }
    }
}

impl Add<i32> for Complex {
    type Output = f64;
    fn add(self, other: i32) -> f64 {
        self.a + self.b + (other as f64)
    }
}


pub fn operator_overriding() {
    println!("===================operator_overriding test begin===================");

    let cp1 = Complex { a: 1f64, b: 2.0 };
    let cp2 = Complex { a: 5.0, b: 8.1 };
    let cp3 = Complex { a: 9.0, b: 20.0 };
    let complex_add_result = cp1 + cp2;
    println!("{:?}\n", complex_add_result);
    println!("{:?}", cp3 + 10i32);

    println!("===================operator_overriding test   end===================");
}