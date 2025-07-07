// 📇 2. Simple Contact List (no saving to file yet)
// Add contacts with name and number, and print them. Store them in a vector
// or array for now.

use std::io;
use std::collections::HashMap;

fn main(){
    let mut choice = 0;
    let mut scores =HashMap::new();
    while choice==0{
        println!("1.Add Contact");
        println!("2.View Contacts");
        println!("3.Exit");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Faild to read input");
        if input.trim() =="1"{
            println!("Enter the number: ");
            let mut number =String::new();
            io::stdin().read_line(&mut number).expect("Faild to take input!");
            println!("Enter the name: ");
            let mut name=String::new();
            io::stdin().read_line(&mut name).expect("Failed to Take input!");
            insert(&mut scores,&mut name,&mut number);
            println!("Contact Added: {} -{}",name.trim(),number.trim());
        }else if input.trim()== "2"{
            display(&scores);
        } else if input.trim()=="3"{
            println!("Thankyou for Interacting with us!");
            choice =1;
        }
    }
}
fn insert(scores:&mut HashMap<String,String>, name:&mut String, number:&mut String){
    scores.insert(name.to_string(),number.to_string());
}
fn display(scores:&HashMap<String,String>){
    println!("Contacts:\n");
    for(name,number) in scores.iter(){
        println!("{:?}:{:?}", name.trim(),number.trim());
    }
}