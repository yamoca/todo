use std::io;

struct Task {
    name: String,
    body: String,
    done: bool,
}

fn main() {
    println!("Hello, world!");
    let tasks: Vec<Task> = vec![];
    tasks.append(create_task());
}

fn create_task() -> Task {
    let mut task_name = String::new();
    io::stdin()
        .read_line(&mut task_name)
        .expect("failed to read line");

    let mut task_body = String::new();
    io::stdin()
        .read_line(&mut task_body)
        .expect("failed to read line");

    let new_task = Task {
        name: String::from("do washing up"),
        body: String::from("Wash and dry dishes, stack dishwasher"),
        done: false
    };
    return new_task;
}
