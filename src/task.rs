use std::fmt;

// #[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub name: String,
    pub is_completed: bool,
    pub desc: String,
    pub due_date: String,
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
