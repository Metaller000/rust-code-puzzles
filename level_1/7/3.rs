/*
Напишите код, который выведет первые N степеней двойки. 
*/

fn main() {        
    print!("первые N степеней двойки: ");
    for res in 1..6{
        print!("{} ", 2.0_f64.powi(res));
    }
    println!();
}