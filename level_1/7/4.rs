/*
Даны три строки:
let txt1: &str = "123";
let txt2: &str = "456";
let txt3: &str = "789";

Сложите значения этих строк как целые числа. 
*/

fn main() {
    let txt1: &str = "123";
    let txt2: &str = "456";
    let txt3: &str = "789";    
    println!("{}: {} ", "summ", txt1.parse::<i16>().unwrap() + txt2.parse::<i16>().unwrap() + txt3.parse::<i16>().unwrap());
}