use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}


fn main() {
    let x = 5;
    let y = MyBox::new(x);

    // *y === *(y.deref())
    println!("y = {}", *y);


    let s1 = MyBox::new(String::from("hello"));
    // 涉及 &String -> &str 的强制转换
    hello(&s1);
    // 如果没有强制转换，则需要这样写
    // hello(&(*s1)[..]);
}
