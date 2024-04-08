fn enum_demo() {
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    println!("{}", value_in_cents(Coin::Penny));
    println!("{}", value_in_cents(Coin::Nickel));
    println!("{}", value_in_cents(Coin::Dime));
    println!("{}", value_in_cents(Coin::Quarter));
}

fn if_else_demo() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");
        10 * n // 这个表达式返回一个 `i32` 类型。
    } else {
        println!(", and is a big number, half the number");
        n / 2 // 这个表达式也必须返回一个 `i32` 类型。
    };
    //   ^ 不要忘记在这里加上一个分号！所有的 `let` 绑定都需要它。

    println!("{} -> {}", n, big_n); // 打印 `5 -> 50`
}

fn if_let_demo() {
    let some_u8_value = Some(3);
    // match some_u8_value {
    //     Some(3) => println!("three"),
    //     _ => (),
    // }

    if let Some(3) = some_u8_value {
        println!("three");
    }
}

fn loop_demo() {
    let mut count = 0;
    loop {
        count += 1;
        println!("count = {}", count);
        if count == 10 {
            break;
        }
    }
}

fn for_demo() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }
}

fn while_demo() {
    let mut count = 0;
    while count != 10 {
        println!("count = {}", count);
        count += 1;
    }
    println!("count = {}", count);
}

fn main() {
    enum_demo();
    if_else_demo();
    if_let_demo();
    loop_demo();
    for_demo();
    while_demo();
}
