#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_assignments)]


mod data;
mod method;
mod trait_test;
mod ownership;
mod borrowing;
mod lifetime;
mod closure;

use data::data_test;
use method::method_test;
use trait_test::trait_test;
use ownership::ownership_test;
use borrowing::borrowing_test;
use lifetime::lifetime_test;
use closure::closure_test;

fn main() {
    //基础数据
    data_test();
    //函数
    method_test();
    //trait
    trait_test();
    //所有权
    ownership_test();
    //借用
    borrowing_test();
    //生命周期
    lifetime_test();
    //闭包
    closure_test();

}
