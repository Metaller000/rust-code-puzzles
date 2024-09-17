/*
Дано дробное число, содержащее количество гигабайт:
let gb: f32 = 35.24;

Переведите это значение в мегабайты, килобайты и байты. 
*/

fn main() {
    let txt: &str = "123456789";
    let mut x: u32 = 0;
    for rs in txt.chars(){
        x=x+rs.to_digit(10).unwrap();    
    }    
    println!("{}: {}", "summ", x)
}
