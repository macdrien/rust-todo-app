use std::io;

fn main() {
    println!("Welcome into your TODO app!");
    
    let mut task_list = vec![];

    loop {
        println!("Task list : {:?}", task_list);
        println!("Please input you command :");

        let mut command = String::new();

        io::stdin().read_line(&mut command).expect("Failed to read lines");

        command.truncate(command.len() - 1);

        if command == "exit" { 
            println!("See you later!");
            break;

        } else if command == "create" {
            println!("Enter the task name");

            let mut task_name = String::new();
            io::stdin().read_line(&mut task_name).expect("Failed to read the line");
            
            task_name.truncate(task_name.len() - 1);
            task_list.push(task_name);

        } else if command == "complete" {
            println!("Enter the name of the task to complete");

            let mut task_name = String::new();
            io::stdin().read_line(&mut task_name).expect("Failed to read the line");
            
            task_name.truncate(task_name.len() - 1);

            let search_result = task_list.iter().position(|task| task == &task_name);

            if  search_result != None {
                task_list.remove(search_result.unwrap());
            } else {
                println!("Task not found");
            }
        }

        println!("---------------------");
    }
}