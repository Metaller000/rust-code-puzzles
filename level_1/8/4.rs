/*
 Дано некоторое число:
let num: i32 = 1234567;

Найдите сумму цифр этого числа. 
*/

fn main() {
    let num: i32 = 1234567;
    let mut x = 0;
    for res in num.to_string().chars(){
        x=x+res.to_digit(10).unwrap();
    }
    println!("{}: {}", "summ", x);
}