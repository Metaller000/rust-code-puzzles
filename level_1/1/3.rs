/*
Дана строка:
let txt: &str = "abcde";

Выведите в консоль последний символ строки. 
*/

fn main() {
    let txt: &str = "abcde";
    println!("{}: {:?}", "последний символ", &txt[txt.len()-1..txt.len()])
}