fn main() {
    let s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string(); // 该方法也可直接用于字符串字面量：
    let s = String::from("initial contents");

    // 字符串相加
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用

    // 多字符串相加
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
}
