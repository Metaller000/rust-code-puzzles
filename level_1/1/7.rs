/*
Дано число:
let num: i16 = 5;

Проверьте, что это число находится в диапазоне от 1 до 10. 
*/

fn main() {
    let num: i16 = 5;
        
    println!("{}: {:?}", "число находится в диапазоне от 1 до 10", num > 0 && num <= 10)
}
