/*
Дано целое число, содержащее номер минуты от 0 до 60:
let num: u8 = 30;

Определите, в какую четверть часа попадает это значение. 
*/

fn main() {
    let num: u8 = 30;    
    println!("{}{}", num/15, "я четверть часа")
}