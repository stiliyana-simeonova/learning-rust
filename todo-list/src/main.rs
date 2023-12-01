use std::io;
use todo_list::task::{Task};

fn command_loop(tasks: &mut Vec<Task>) {
    loop {
        println!("Enter command (add, list, done, delete, exit)");

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line!");

        let command = command.trim();

        match command {
            "add" => {
                //handle add command
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin()
                    .read_line(&mut title)
                    .expect("Failed to read line!");
                add_task(tasks, title.trim().to_string());
            },
            "list" => {
                //handle list command
                println!("Listing");
                list_tasks(tasks);
            },
            "done" => {
                //mark command as done
                println!("Enter task id:");
                let mut id = String::new();
                io::stdin()
                    .read_line(&mut id)
                    .expect("Failed to read line!");
                let id: usize = id.trim().parse().expect("Input is not integer!");
                mark_task_done(tasks,id);
            },
            "delete" => {
                //deleting task
                println!("Deleting");
            },
            "exit" => break,
            _ => println!("Unknonw command"),
        }
    }
}

fn add_task(tasks: &mut Vec<Task>, title: String) {
    let new_task = Task::new(tasks.len()+1, title);
    tasks.push(new_task);
}

fn list_tasks(tasks: &[Task]) {
    for task in tasks {
        println!("{}: {} [{}]", task.id, task.title, task.status.as_str());
    }
}

fn mark_task_done(tasks: &mut Vec<Task>, id: usize) {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.complete();
    }
    else {
        println!("Task with ID {} not found.", id);
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    command_loop(&mut tasks);
}
