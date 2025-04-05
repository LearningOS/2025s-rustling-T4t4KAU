fn main() {
    let mut res = 42;
    let option = Some(12);

    res += option.unwrap_or(0);

    println!("{}", res);
}