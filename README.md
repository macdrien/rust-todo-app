# TODO APP

This is my first TODO APP in Rust.

Current version : 1.0.0  
Last release : 1.0.0

## Installation

To use the application, you have to install [rust](https://www.rust-lang.org/tools/install) and [crago](https://crates.io/).
After that, download the project. In the root folder, enter the command cargo build.  
To start the application, juste enter cargo run.

## Desription

This application is a command line application.

You can create and complete tasks. To do that, you have to write the corresponding command.

The application is a big loop. Each loop start a the list of current tasks. You can enter a command just after to manage the list or leave the application.

**Careful**: The application does not save the task list for now. So, when you leave the application, you will loose your opened tasks.

## Commands

### Create

The create command allows you to create a new task. To use it, just write 'create' when the application asks you to enter a command.  
It will asks you to enter your task's name next (in 2 commands).

### Complete

Complete a task removes it from the list. You have to enter 'complete' to activate it. Next, enter the name of the task to complete.

### Exit

Exit allows you to leave the application easily. Just enter the command 'exit' to do it.
