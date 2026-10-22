fn main() {
    println!("Hello");
    let res: bool = is_even(10);
    println!("{}", res);
}
fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return false;
}
