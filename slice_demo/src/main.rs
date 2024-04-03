fn first_word(s: &str) -> &str {
    let s_bytes = s.as_bytes();
    for (i, &item) in s_bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}

fn main() {
    let mut str = String::from("hello world");
    let first_word = first_word(&str);
    println!("{:?}", first_word);
}
