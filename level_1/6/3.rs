/*
Даны три символа:
let chr1: char = '1';
let chr2: char = '2';
let chr3: char = '3';

Сложите значения этих символов как целые числа. 
*/

fn main() {    
    let chr1: char = '1';
    let chr2: char = '2';
    let chr3: char = '3';
    println!("{}: {}", "summ", chr1.to_digit(10).unwrap() + chr2.to_digit(10).unwrap() + chr3.to_digit(10).unwrap());
}