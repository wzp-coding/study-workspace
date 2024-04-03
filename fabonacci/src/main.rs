fn generate_fibonacci(n: i32) -> Vec<i32> {
    let mut fabonacci = Vec::new();
    fabonacci.push(0);
    fabonacci.push(1);
    for i in 2..n {
        let new_value = fabonacci[i as usize - 1] + fabonacci[i as usize - 2];
        fabonacci.push(new_value);
    }
    fabonacci
}

fn main() {
    let fabonacci = generate_fibonacci(10);
    println!("{:?}", fabonacci);
}
