use libc::c_int;
use libc::c_void;
use libc::size_t;
use libc::c_schar;
use std::os::raw::c_char;
use std::ptr;

use std::ffi::CStr;
use std::ffi::CString;
/*#[link(name = "yourlib")]
extern {
    fn your_func(arg1: c_int, arg2: *mut c_void) -> size_t;// 声明ffi函数
    fn your_func2(arg1: c_int, arg2: *mut c_void) -> size_t;
    static ffi_global: c_int; // 声明ffi全局变量
}*/
extern "C" fn callback(a: c_int) {
    // 这个函数是传给c调用的
    println!("C call back : hello {}!", a);
}

#[link(name = "hello")]
extern {
    fn run_callback(data: i32, cb: extern fn(i32));
    fn char_func() -> *mut c_char;
    fn my_printer(s: *const c_char);
}

fn get_string() -> String {
    unsafe {
        let raw_string: *mut c_char = char_func();
        let cstr = CStr::from_ptr(raw_string);
        cstr.to_string_lossy().into_owned()
    }
}

pub fn ffi_test() {
    println!("===================ffi test begin===================");
    /* let result: size_t = unsafe {
         your_func(1 as c_int, Box::into_raw(Box::new(3)) as *mut c_void)
     };*/

    unsafe {
        run_callback(666 as i32, callback); // 打印 1
        println!("c return str={}", get_string());
    }

    let c_to_print = CString::new("Hello, world!").unwrap();
    unsafe {
        my_printer(c_to_print.as_ptr());
    }

    println!("===================ffi test   end===================");
}