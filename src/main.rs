use std::io;

fn main() {
    println!("Welcome into your TODO app!");
    
    loop {
        println!("Please input you command :");

        let mut command = String::new();

        io::stdin().read_line(&mut command).expect("Failed to read lines");

        command.truncate(command.len() - 1);

        if command == "exit" { 
            println!("See you later!");
            break;
        }
    }
}