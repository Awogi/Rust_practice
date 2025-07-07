// 🧠 1. First Word Extractor CLI
// Build a small command-line app that takes user input and returns the first 
// word of the sentence.
use std::io;
fn main(){
    println!("Please enter a sentence: ");
    let mut input= String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let index;
    index= find_first_word(&input);
    println!("The sentence is {:?}.",input.trim());
    println!("the first word of the sentence is {:?}.",&input[..index]);

}
fn find_first_word(s:&String)->usize{
    let byte= s.as_bytes();
    for(i,&item) in byte.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }
    return s.len(); 
}


