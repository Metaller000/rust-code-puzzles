/*
Расскажите, сколько байт занимает тип i8. 
*/
use std::mem;

fn main() {
    println!("{}: {} байт", "тип i8 занимает байт", mem::size_of::<i8>())
}
