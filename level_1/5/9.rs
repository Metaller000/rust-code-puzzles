/*
Дано целое число, содержащее номер дня от 1 до 31:
let num: u16 = 1;

Определите, в какую декаду месяца попадает этот день. 
*/

fn main() {
    let num: u16 = 1;
    println!("{}", (num-1)/10+1)
}
