use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main(){
    let secret_number = rand::thread_rng().gen_range(1,10);
    loop{
        let mut guess = String::new();
        println!("Guess a number:");
    
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read");

        let guess:u32 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };

        match guess.cmp(&secret_number){
            Ordering::Less => println!("{}","Too Small".red()),
            Ordering::Greater => println!("{}","Too Big".red()),
            Ordering::Equal => {
                println!("{}","Correct".green());
                break;
            }
        }
    }
}
