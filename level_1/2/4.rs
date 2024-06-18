/*
Дано целое число:
let num: i32 = 123;

Выведите в консоль сумму первой и последней цифры этого числа. 
*/

fn main() {
    let num: i32 = 123;
    let n = num.to_string();
    let nf = &n[..1].parse::<i8>().unwrap();
    let nl = &n[n.len()-1..].parse::<i8>().unwrap();
    println!("{}: {}", "сумма первой и последней цифры", nf+nl)
}