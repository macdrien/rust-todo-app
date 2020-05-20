use std::vec::Vec;
use std::io;

fn main() {
    println!("Welcome into your TODO app!");
    
    loop {
        print!("Please enter your command : ");
        
            let mut command = String::new();
        
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read the line");

        println!("You wrote : {}", command);

        if command == "exit" { 
            println!("See you later!");
            break;
        }
    }
}

#[derive(Debug)]
struct Todo {
    title: String,
}

#[derive(Debug)]
struct TodoList {
    list: Vec<Todo>,
}