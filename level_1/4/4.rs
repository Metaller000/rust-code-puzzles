/*
Дана некоторая строка:
let txt: &str = "abcde";

Переберите и выведите в консоль по очереди все ее символы. 
*/

fn main() {
    let txt: &str = "abcde";
    for res in txt.chars() {
        print!("{} ", res);
    }    
    println!()
}