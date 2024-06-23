/*
Даны два целых числа:
let num1: u16 = 36;
let num2: u16 = 12;

Выведите в консоль большее из этих чисел. 
*/

fn main() {
    let num1: u16 = 36;
    let num2: u16 = 12;
    let x = | a, b | if a >= b {a} else {b};
    println!("{}: {}", "большее из чисел", x(num1, num2))
}
