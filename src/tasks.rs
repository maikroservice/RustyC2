// List/Vector of individual Tasks
#[derive(Debug, Default)]
pub struct Tasklist {
    pub tasks: Vec<Task>,
}
// potential status of a task
#[derive(Debug, PartialEq)]
pub enum TaskStatus {
    Waiting,
    Running,
    Finished,
}

// a single Task
#[derive(Debug)]
pub struct Task {
    pub id: uuid::Uuid,
    pub command: String,
    pub status: TaskStatus,
    pub output: String,
}

impl Default for Task {
    fn default() -> Self {
        let uid = uuid::Uuid::new_v4();
        Task {
            id: uid,
            command: "".to_string(),
            status: TaskStatus::Waiting,
            output: "".to_string(),
        }
    }
}

impl Tasklist {
    fn find_first_waiting_task_index(self) -> i32 {
        for (i, item) in self.tasks.iter().enumerate() {
            match item.status {
                Waiting => i.try_into().unwrap(),
                _ => -1,
            }
        }
    }
}
