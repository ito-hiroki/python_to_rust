fn main() {
    let mut a = 1;
    let mut b = 1;
    println!("{}", a);
    println!("{}", b);
    for _ in 0..30 {
        let tmp = a + b;
        println!("{}", tmp);
        a = b;
        b = tmp;
    }
}
