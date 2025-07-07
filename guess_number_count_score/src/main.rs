// 🎲 3. Number Guessing Game with Score Counter
// Extend the classic guessing game: count how many tries it takes, and show a message 
// based on performance.

use std::io;
use rand::Rng;

fn main(){
    println!("Let's play a guessing game!");
    let mut score = 0;
    let mut attempts = 0;
    let mut guess=String::new();
    let mut match=false;
    let secret_number = rand::thread_rng().gen_range(1..=100);
    while match==false{
        attempt
        println!("Enter your guess(1-100):");
        io::stdin().read_line(&mut guess).expect("faild to read your guess!");
        attempts+=1;
        if(guess.trim()==secret_number.to_string()){
            score += 1;
            println
        }
    }
}
