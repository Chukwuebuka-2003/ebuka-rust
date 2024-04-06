// How to convert a temperature between Celsius and Fahrenheit in Rust

fn fah_to_cel(fah: f64) -> f64 {
    (fah - 32.0) * 5.0 / 9.0
}

fn cel_to_fah(cel: f64) -> f64 {
    cel * 9.0 / 5.0 + 32.0
}

fn main() {
    let fah_temp: f64 = 98.0;
    let cel_temp = fah_to_cel(fah_temp);

    println!("{}°F is {}°C", fah_temp, cel_temp);

    let cel_temp: f64 = 37.0;
    let fah_temp = cel_to_fah(cel_temp);

    println!("{}°C is {}°F", cel_temp, fah_temp);
}
