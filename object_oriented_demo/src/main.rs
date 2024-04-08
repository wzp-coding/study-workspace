fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s); // 对 String "hello" 进行不可变借用
    println!("The length of '{}' is {}.", s, len); // 正常输出 "The length of 'hello' is 5."
}

fn calculate_length(s: &String) -> usize { // 函数参数 s 是对 String 的不可变借用
    s.len() // 返回 String 的长度，不会获取所有权
} // 不可变借用结束，没有释放任何所有权