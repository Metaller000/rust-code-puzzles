/*
Даны два слова.
let word1: &str = "abc";
let word2: &str = "ade";

Проверьте, что первые буквы этих слов совпадают. 
*/

fn main() {
    let word1: &str = "abc";
    let word2: &str = "ade";

    println!("{}: {:?}", "первый символ строк совпадает", &word1[..1]==&word2[..1] )
}