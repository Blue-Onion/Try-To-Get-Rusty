fn main() {
    println!("{}", fibo(21))
}
fn fibo(num: i32) -> i32 {
    let mut fis = 0;
    let mut sec = 1;
    if num == 0 {
        return fis;
    }
    if num == 1 {
        return sec;
    }
    for _ in 1..num - 2 {
        let tmp = sec;
        sec += fis;
        fis = tmp
    }
    sec
}
