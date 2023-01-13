use declarative_macro::*;
use serde::Deserialize;
make_public! {
    pub(crate) struct A {
        a : u8,
        b: u8,
    }
}

make_public_v2! {
    #[derive(Debug, Deserialize)]
    pub(crate) struct B {
        #[serde(rename = "A")]
        a : u8,
        b: u8,
    }
}

fn main() {
    let v = add_v1!(2, 2);
    println!("v = {}", v);
    let v = add_as_v1!(2, 2, u8);
    println!("v = {}", v);
    let v = add_v2!(1, 2, 3, 4);
    println!("v = {}", v);
    let v = add_v3!(1);
    println!("v = {}", v);
    let v = add_v3!(1, 2);
    println!("v = {}", v);
    let v = add_v3!(1 as u8, 2, 3, 4);
    println!("v = {}", v);
    let v: u8 = add_v3!();
    println!("v = {}", v);

    let b = B { a: 1, b: 2 };
    println!("b = {:?}", b);

    // let a = A { a: 1, b: 2 };
    // println!("a = {:?}", a);
}
