/*
Дано дробное число, содержащее количество гигабайт:
let gb: f32 = 35.24;

Переведите это значение в мегабайты, килобайты и байты.
*/

fn main() {
    let gb: f32 = 35.24;

    println!("{}: {}, {}, {}", "мегабайты, килобайты и байты",  2.0_f32.powf(10.0) * gb, 2.0_f32.powf(20.0) * gb, 2.0_f32.powf(30.0) * gb)
}
