use declarative_macro::*;

fn some_work(i: i64, j: i64) -> Result<i64, String> {
    if i + j > 2 {
        Ok(i)
    } else {
        Err("error".into())
    }
}

fn main() -> Result<(), String> {
    let v = ok_or_return!(some_work(1, 2));
    println!("v = {}", v);
    let v = ok_or_return!(some_work(1, 3));
    println!("v = {}", v);
    let v = ok_or_return2!(some_work(1, 2));
    println!("v = {}", v);
    // 也能使用{}，但是这样就不能使用分号了。
    let v = ok_or_return2! {some_work(1, 1)};
    println!("v = {}", v);
    Ok(())
}
