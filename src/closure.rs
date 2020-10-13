use std::thread;
use std::time::Duration;

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

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    let x = 4;

    let equal_to_x = move |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
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

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}