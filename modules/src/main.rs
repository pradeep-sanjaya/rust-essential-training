use rand::prelude::*;
use std::io;

fn main() {

    let answer_exit = String::from("y");
    
    loop {
        let mut buffer_answer = String::new();
        let mut buffer = String::new();

        let answer_number:i32 = thread_rng().gen_range(1..10);
        println!("Enter a number: ");
        io::stdin().read_line(&mut buffer);
    
        let number = buffer.trim().parse::<i32>().unwrap();
        println!("Number: {}", buffer);
    
        if (number == answer_number) {
            println!("You won!")
        } else {
            println!("You lose, Number is {}", answer_number);
        }
    
        println!("Do you wan to exit?(y)");
        io::stdin().read_line(&mut buffer_answer);
    
        if buffer_answer.trim().to_lowercase() == answer_exit {
            break;
        }
    }
}
