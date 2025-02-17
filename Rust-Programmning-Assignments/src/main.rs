fn farenheit_to_celsius(farenheit: f64) -> f64 {
    (farenheit - 32.0) * 5.0 / 9.0
}
fn celsius_to_farenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}
fn main() {
    let mut farenheit:f64 = 32.0;
    let celsius = farenheit_to_celsius(farenheit);
    println!("{} farenheit is {} celsius", farenheit, celsius);
    while farenheit <= 37.0{
        farenheit += 1.0;
        let celsius = farenheit_to_celsius(farenheit);
        println!("{} farenheit is {} celsius", farenheit, celsius);
    }
}
