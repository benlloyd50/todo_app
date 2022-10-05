use std::{
    fmt, fs,
    io::{self, BufReader, Read},
};

use serde::Deserialize;

const TASKS_FILE_PATH: &str = "data/tasks.json";

#[derive(Deserialize, Clone)]
struct Task {
    name: String,
    is_completed: bool,
    desc: String,
    due_date: String,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "| {: ^23} => {: <72} Due: {: ^10} |",
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

    // Display any existing, non complete tasks
    print_tasks(
        all_tasks
            .iter()
            .filter(|t| !t.is_completed)
            .cloned() //prevents a copy of the vec from being discarded and recreated with collect
            .collect(),
    );

    let mut input_string = String::new();
    loop {
        input_string.clear();
        io::stdin()
            .read_line(&mut input_string)
            .expect("Input was invalid");
        if input_string.trim().eq_ignore_ascii_case("finish") {
            finish_task(&mut all_tasks);
        } else if input_string.trim().eq_ignore_ascii_case("quit") {
            break;
            //Save new tasks to file
        }
    }
}

fn print_tasks(active_tasks: Vec<Task>) {
    println!("┌{:-^117}┐", "Ongoing Tasks");
    for task in active_tasks.iter() {
        println!("{}", task)
    }
    println!("└{:-<117}┘", "What's next?");
}

fn finish_task(all_tasks: &mut Vec<Task>) {
    println!("What task did you finish?");
}
