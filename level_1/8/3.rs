/*
Дано целое число, содержащее количество байт:
let b: i64 = 7435421243;

Переведите это значение в гигабайты, мегабайты и килобайты. 
*/

fn main() {            
    let b: i64 = 7435421243;
    println!("{}: {}, {}, {}","гигабайты, мегабайты и килобайты", b / 2_i64.pow(30), b / 2_i64.pow(20), b / 2_i64.pow(10));    
}