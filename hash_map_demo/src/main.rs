fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // 500 不会被插入，因为 Yellow 已经存在了
    scores.entry(String::from("Yellow")).or_insert(500);
    // Orange 不存在，所以 <Orange, 50> 会被插入
    scores.entry(String::from("Orange")).or_insert(50);

    // 访问 HashMap 的值
    scores.get(&String::from("Blue")); // 10
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 统计单词出现次数
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    // {"world": 2, "hello": 1, "wonderful": 1}

    // Vector => HashMap
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // 所有权
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 这里 field_name 和 field_value 不再有效，
}
