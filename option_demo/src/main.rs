fn plus_one(n: Option<i32>) -> Option<i32> {
    match n {
        None => None,
        Some(n) => Some(n + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}", six);
    println!("{:?}", none);
}
