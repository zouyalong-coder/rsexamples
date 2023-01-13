use procedural_macro::*;

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

// #[derive(Trait)]
// struct MyStruct {}
