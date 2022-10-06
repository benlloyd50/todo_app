use std::{
    fmt, fs,
    io::{self, BufReader, Read},
};

use serde::{Deserialize, Serialize};

const TASKS_FILE_PATH: &str = "data/tasks.json";

#[derive(Serialize, Deserialize, Clone)]
struct Task {
    name: String,
    is_completed: bool,
    desc: String,
    due_date: String,
}

impl Default for Task {
    fn default() -> Self {
        Task {
            name: "?".to_string(),
            is_completed: false,
            desc: "none".to_string(),
            due_date: "none".to_string(),
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "│ {: ^26} => {: <69} Due: {: ^10} │",
            self.name, self.desc, self.due_date
        )
    }
}

fn main() {
    // Load existing tasks if a file exists with data
    let tasks_file = match fs::File::open(TASKS_FILE_PATH) {
        Ok(file) => file,
        Err(_) => match fs::File::create(TASKS_FILE_PATH) {
            Ok(file) => file,
            Err(_) => {
                panic!(
                    "Could not create file at {}, cannot run TODO app",
                    TASKS_FILE_PATH
                );
            }
        },
    };
    let mut task_string = String::new();
    let mut br = BufReader::new(tasks_file);
    br.read_to_string(&mut task_string)
        .expect("Nonvalid UTF-8 character found in file");
    let mut all_tasks: Vec<Task> =
        serde_json::from_str(&task_string).expect("Json could not be deserialized");

    let mut input_string = String::new();
    let mut should_print: bool = true;
    loop {
        //update active tasks and display to user
        if should_print {
            let active_tasks: Vec<Task> = all_tasks
                .iter()
                .filter(|t| !t.is_completed)
                .cloned() //prevents a copy of the vec from being discarded and recreated with collect
                .collect();
            print_tasks(&active_tasks, "Ongoing Tasks");
        } else {
            should_print = true;
        }

        input_string.clear();
        io::stdin()
            .read_line(&mut input_string)
            .expect("Input was invalid");
        if input_string.trim().eq_ignore_ascii_case("finish") {
            finish_task(&mut all_tasks);
        } else if input_string.trim().eq_ignore_ascii_case("create") {
            create_task(&mut all_tasks);
        } else if input_string.trim().eq_ignore_ascii_case("delete") {
            delete_task(&mut all_tasks);
        } else if input_string.trim().eq_ignore_ascii_case("history") {
            print_tasks(&all_tasks, "Every Task");
            should_print = false;
        } else if input_string.trim().eq_ignore_ascii_case("quit") {
            save_all_tasks(&all_tasks);
            break;
        }
    }
}

fn print_tasks(tasks: &Vec<Task>, header: &str) {
    println!("┌{:─^117}┐", header);
    for task in tasks.iter() {
        println!("{}", task)
    }
    println!("└{:─<117}┘", "What's next?");
}

fn finish_task(all_tasks: &mut Vec<Task>) {
    println!("What task did you finish? (index)");
    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Input was invalid");
    let idx = match input_string.trim().parse::<i32>() {
        Ok(idx) => idx,
        Err(err) => {
            panic!("Can not do that thing sorry im out, {}", err);
        }
    };
    // all_tasks.remove(idx as usize);
    all_tasks[idx as usize].is_completed = true;
}

fn delete_task(all_tasks: &mut Vec<Task>) {
    println!("What task did you want to delete? (index)");
    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Input was invalid");
    let idx = match input_string.trim().parse::<i32>() {
        Ok(idx) => idx,
        Err(err) => {
            panic!("Can not do that thing sorry im out, {}", err);
        }
    };
    all_tasks.remove(idx as usize);
}

fn create_task(all_tasks: &mut Vec<Task>) {
    let fields = vec!["name", "description", "due date"];
    let mut field = String::new();
    let mut new_task: Task = Default::default();
    for i in 0..3 {
        field.clear();
        println!("Enter a {} for the new task", fields[i]);
        io::stdin()
            .read_line(&mut field)
            .expect("Input was invalid");
        field = field.trim().to_string();
        if i == 0 {
            new_task.name = field.clone();
        } else if i == 1 {
            new_task.desc = field.clone();
        } else if i == 2 {
            new_task.due_date = field.clone();
        }
    }
    all_tasks.push(new_task);
}

fn save_all_tasks(all_tasks: &Vec<Task>) {
    let json_string = match serde_json::to_string_pretty(&all_tasks) {
        Ok(val) => val,
        Err(err) => {
            panic!("Could not convert tasks into json {}", err);
        }
    };
    fs::write(TASKS_FILE_PATH, json_string)
        .expect(&format!("Could not write to the file {}", TASKS_FILE_PATH));
}
