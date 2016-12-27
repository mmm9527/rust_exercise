#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![feature(box_syntax, box_patterns)]
#![feature(ptr_eq)]

extern crate rand;
extern crate libc;
extern crate rustc_serialize;
// 引入rustc_serialize模块

mod data;
mod method;
mod trait_test;
mod ownership;
mod borrowing;
mod lifetime;
mod closure;
mod collections;
mod iterator;
mod error_handle;
mod io_test;
mod macro_test;
mod heap_stack_box;
mod smart_pointer;
mod common_trait;
mod concurrent;
mod unsafe_raw_pointer;
mod ffi;
mod operator_overriding;
mod attr_compiler_options;
mod any_reflect;
mod common_data_structure;
mod std_test;
mod actual_exercise;

use data::data_test;
use method::method_test;
use trait_test::trait_test;
use ownership::ownership_test;
use borrowing::borrowing_test;
use lifetime::lifetime_test;
use closure::closure_test;
use collections::collection_test;
use iterator::iterator_test;
use error_handle::error_handle_test;
use io_test::io_test;
use macro_test::macro_test;
use heap_stack_box::heap_stack_box_test;
use smart_pointer::smart_pointer_test;
use common_trait::common_trait_test;
use concurrent::concurrent_test;
use unsafe_raw_pointer::unsafe_raw_pointer;
use ffi::ffi_test;
use operator_overriding::operator_overriding;
use attr_compiler_options::compiler_options;
use any_reflect::any_reflect_test;
use common_data_structure::common_data_structure;
use std_test::std_test;
use actual_exercise::actual_exercise;



fn main() {
    //基础数据
    //    data_test();
    //函数
    //    method_test();
    //trait
    //    trait_test();
    //所有权
    //    ownership_test();
    //借用
    //    borrowing_test();
    //生命周期
    //    lifetime_test();
    //闭包
    //    closure_test();
    //集合类型
    //    collection_test();
    //迭代器
    //    iterator_test();
    //异常处理 Option、Result和异常处理
    //    error_handle_test();
    //基本输入输出
    //    io_test();
    //宏
    //    macro_test();
    //堆、栈、box
    //    heap_stack_box_test();
    //几种智能指针
    //    smart_pointer_test();
    //类型系统中的几个常见 trait
    //    common_trait_test();
    //并发，并行，多线程编程
    //    concurrent_test();
    //unsafe 、原始指针
    //    unsafe_raw_pointer();
    //FFI
    //    ffi_test();
    //运算符重载
    //    operator_overriding();
    //属性和编译器参数
    //    compiler_options();
    //any与反射
    //    any_reflect_test();
    //常用数据结构
    //    common_data_structure();
    //标准库介绍
    //    std_test();
    //实战练习
    actual_exercise();
}

