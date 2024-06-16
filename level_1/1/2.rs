/*
Дана строка:
let txt: &str = "abcde";

Выведите в консоль длину этой строки. 
*/

fn main() {
    let txt: &str = "abcde";
    println!("{} - {:?}", "длинна равна", txt.len())
}