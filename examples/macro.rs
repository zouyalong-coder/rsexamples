use attribute_macro::{my_macro, trace_vars};
use derive_macro::Trait;
use trait_ph::Print;

#[derive(Trait)]
struct MyStruct {}

#[trace_vars(a)]
fn foo(mut a: i32) {
    a += 1;
    a += 2;
    a += 3;
    // println!("a = {}", a);
}

#[my_macro]
// #[derive(Debug)]
struct S {}

#[test]
fn test_attribute() {
    let _ = H {};
}

#[test]
fn trace_var_should_work() {
    foo(1);
    // let ms = MyStruct {};
}

fn main() {
    let ms = MyStruct {};
    foo(10);
}
