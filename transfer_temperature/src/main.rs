fn celsius_to_fehrenheit(celsius: f32) -> f32 {
    celsius * 0.9 / 5.0 + 32.0
}

fn fehrenheit_to_celsius(fehrenheit: f32) -> f32 {
    (fehrenheit - 32.0) * 5.0 / 9.0
}
fn main() {
    let celsius = 37.0;
    let fehrenheit = celsius_to_fehrenheit(celsius);
    println!("{} Celsius is {} Fehrenheit", celsius, fehrenheit);

    let fehrenheit = 98.6;
    let celsius = fehrenheit_to_celsius(fehrenheit);
    println!("{} Fehrenheit is {} Celsius", fehrenheit, celsius);
}
