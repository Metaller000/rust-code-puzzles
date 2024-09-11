/*
Даны три целых числа:
let num1: u16 = 36;
let num2: u16 = 24;
let num3: u16 = 12;

Выведите в консоль большее из этих чисел. 
*/

fn main() {
    let num1: u16 = 36;
    let num2: u16 = 24;
    let num3: u16 = 12;

    let mut numx: u16;
    if num1 < num2 {
        numx = num2;
    } else {
        numx = num1;
    }

    if numx < num3 {
        numx = num3;
    } 

    println!("{}", numx)
}
