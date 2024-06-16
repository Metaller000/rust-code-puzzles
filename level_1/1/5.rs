/*
Дано слово:
let word: &str = "день";

Получите его последнюю букву. Если слово заканчивается на мягкий знак, то получите предпоследнюю букву. 
*/

fn main() {
    let word: &str = "день";    
    let mut ch = word.chars().last().unwrap();
    if ch == 'ь' {
        ch = word.chars().rev().nth(1).unwrap();
    }
    println!("{}: {:?}", "последняя буква", ch)
}
