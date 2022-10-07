mod task;
use rocket::log::private::Log;
use rocket_db_pools::{sqlx::SqlitePool, Connection, Database};
use task::Task;
#[macro_use]
// allows us the privelege of using rocket macros globally since we'll use them everywhere
extern crate rocket;

#[get("/task/<task_name>")]
fn index(task_name: String) -> String {
    //how do we make all_tasks persistent? with a db for different sessions? then how do we load it quick? test speed of db!
    let mut all_tasks: Vec<Task> = Vec::new();
    let simple_task = Task {
        name: task_name,
        ..Task::default()
    };
    all_tasks.push(simple_task);
    make_task_box_string(&all_tasks, "Keep On Working!")
}

#[get("/<name>")]
async fn read(mut db: Connection<Tasks>, name: String) {
    // sqlx::query!("SELECT name FROM task WHERE name = ?", id)
    //     .fetch_one(&mut *db)
    //     .map_ok(|r| Log(r.content))
    //     .await
}

#[derive(Database)]
#[database("task_db")]
struct Tasks(SqlitePool);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Tasks::init())
        .mount("/", routes![index])
}

// use std::{
//     fmt, fs,
//     io::{self, BufReader, Read},
// };

// use serde::{Deserialize, Serialize};

// const TASKS_FILE_PATH: &str = "data/tasks.json";

// fn main() {
//     // Load existing tasks if a file exists with data
//     let tasks_file = match fs::File::open(TASKS_FILE_PATH) {
//         Ok(file) => file,
//         Err(_) => match fs::File::create(TASKS_FILE_PATH) {
//             Ok(file) => file,
//             Err(_) => {
//                 panic!(
//                     "Could not create file at {}, cannot run TODO app",
//                     TASKS_FILE_PATH
//                 );
//             }
//         },
//     };
//     let mut task_string = String::new();
//     let mut br = BufReader::new(tasks_file);
//     br.read_to_string(&mut task_string)
//         .expect("Nonvalid UTF-8 character found in file");
//     let mut all_tasks: Vec<Task> =
//         serde_json::from_str(&task_string).expect("Json could not be deserialized");

//     let mut input_string = String::new();
//     let mut should_print: bool = true;
//     loop {
//         //update active tasks and display to user
//         if should_print {
//             let active_tasks: Vec<Task> = all_tasks
//                 .iter()
//                 .filter(|t| !t.is_completed)
//                 .cloned() //prevents a copy of the vec from being discarded and recreated with collect
//                 .collect();
//             print_tasks(&active_tasks, "Ongoing Tasks");
//         } else {
//             should_print = true;
//         }

//         input_string.clear();
//         io::stdin()
//             .read_line(&mut input_string)
//             .expect("Input was invalid");
//         if input_string.trim().eq_ignore_ascii_case("finish") {
//             finish_task(&mut all_tasks);
//         } else if input_string.trim().eq_ignore_ascii_case("create") {
//             create_task(&mut all_tasks);
//         } else if input_string.trim().eq_ignore_ascii_case("delete") {
//             delete_task(&mut all_tasks);
//         } else if input_string.trim().eq_ignore_ascii_case("history") {
//             print_tasks(&all_tasks, "Every Task");
//             should_print = false;
//         } else if input_string.trim().eq_ignore_ascii_case("quit") {
//             save_all_tasks(&all_tasks);
//             break;
//         }
//     }
// }

// fn print_tasks(tasks: &Vec<Task>, header: &str) {
//     println!("┌{:─^117}┐", header);
//     for task in tasks.iter() {
//         println!("{}", task)
//     }
//     println!("└{:─<117}┘", "What's next?");
// }

fn make_task_box_string(tasks: &Vec<Task>, header: &str) -> String {
    let mut bs = String::new();
    bs += format!("┌{:─^117}┐\n", header).as_str();
    for task in tasks.iter() {
        bs += format!("{}\n", task).as_str();
    }
    bs += format!("└{:─<117}┘\n", "What's next?").as_str();
    bs
}

// fn finish_task(all_tasks: &mut Vec<Task>) {
//     println!("What task did you finish? (index)");
//     let mut input_string = String::new();
//     io::stdin()
//         .read_line(&mut input_string)
//         .expect("Input was invalid");
//     let idx = match input_string.trim().parse::<i32>() {
//         Ok(idx) => idx,
//         Err(err) => {
//             panic!("Can not do that thing sorry im out, {}", err);
//         }
//     };
//     // all_tasks.remove(idx as usize);
//     all_tasks[idx as usize].is_completed = true;
// }

// fn delete_task(all_tasks: &mut Vec<Task>) {
//     println!("What task did you want to delete? (index)");
//     let mut input_string = String::new();
//     io::stdin()
//         .read_line(&mut input_string)
//         .expect("Input was invalid");
//     let idx = match input_string.trim().parse::<i32>() {
//         Ok(idx) => idx,
//         Err(err) => {
//             panic!("Can not do that thing sorry im out, {}", err);
//         }
//     };
//     all_tasks.remove(idx as usize);
// }

// fn create_task(all_tasks: &mut Vec<Task>) {
//     let fields = vec!["name", "description", "due date"];
//     let mut field = String::new();
//     let mut new_task: Task = Default::default();
//     for i in 0..3 {
//         field.clear();
//         println!("Enter a {} for the new task", fields[i]);
//         io::stdin()
//             .read_line(&mut field)
//             .expect("Input was invalid");
//         field = field.trim().to_string();
//         if i == 0 {
//             new_task.name = field.clone();
//         } else if i == 1 {
//             new_task.desc = field.clone();
//         } else if i == 2 {
//             new_task.due_date = field.clone();
//         }
//     }
//     all_tasks.push(new_task);
// }

// fn save_all_tasks(all_tasks: &Vec<Task>) {
//     let json_string = match serde_json::to_string_pretty(&all_tasks) {
//         Ok(val) => val,
//         Err(err) => {
//             panic!("Could not convert tasks into json {}", err);
//         }
//     };
//     fs::write(TASKS_FILE_PATH, json_string)
//         .expect(&format!("Could not write to the file {}", TASKS_FILE_PATH));
// }
