/*
Даны три символа:
let chr1: char = 'a';
let chr2: char = 'b';
let chr3: char = 'c';

Слейте эти символы в строку:
std::string "abc"
*/

fn main() {
    let chr1: char = 'a';
    let chr2: char = 'b';
    let chr3: char = 'c';
    
    let mut st = String::new();
    st.push(chr1);
    st.push(chr2);
    st.push(chr3);

    println!("{}", st)
}
