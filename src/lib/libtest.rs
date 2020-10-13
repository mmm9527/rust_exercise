#![crate_type = "dylib"]
extern crate libc;

use self::libc::c_void;
use std::any::Any;

#[no_mangle]
pub extern "C" fn test() {
    println!("hello lib !");
}

#[derive(Debug)]
struct Foo<T> {
    t: T
}

#[no_mangle]
pub extern "C" fn new_foo_vec() -> *const c_void {
    Box::into_raw(Box::new(Foo { t: vec![1, 2, 3] })) as *const c_void
}

#[no_mangle]
pub extern "C" fn new_foo_vec2() -> *const c_void {
    Box::into_raw(Box::new(Box::new(Foo { t: vec![1, 2, 3] }) as Box<Any>)) as *const c_void
}

#[no_mangle]
pub extern "C" fn new_foo_int() -> *const c_void {
    Box::into_raw(Box::new(Foo { t: 1 })) as *const c_void
}

#[no_mangle]
pub extern "C" fn new_foo_int2() -> *const c_void {
    Box::into_raw(Box::new(Box::new(Foo { t: 1 }) as Box<Any>)) as *const c_void
}

fn push_foo_element(t: &mut Foo<Vec<i32>>) {
    t.t.push(1);
}

pub extern "C" fn push_foo_element_c(foo: *mut c_void) {
    let foo2 = unsafe {
        &mut *(foo as *mut Foo<Vec<i32>>)
    };
    push_foo_element(foo2);
}


pub extern "C" fn push_foo_element_c2(foo: *mut c_void) {
    let foo2 = unsafe {
        &mut *(foo as *mut Box<Any>)
    };
    let foo3: Option<&mut Foo<Vec<i32>>> = foo2.downcast_mut();
    if let Some(value) = foo3 {
        push_foo_element(value);
    }
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(true,"12321321");
    }
}


