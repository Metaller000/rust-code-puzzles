/*
Дано целое число. Проверьте, что первая и последняя цифры этого числа совпадают. 
*/

fn main() {
    let num= 91;
    println!("{}: {}", "первая и последняя цифры числа совпадают", num.to_string()[..1]==num.to_string()[num.to_string().len()-1..])
}