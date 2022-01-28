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
    Completed,
    Failed,
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

/* in a tasklist you have different Kommandos as tasks
 a task can have 3 different statuses -> waiting, running, finished
 find the first waiting task in order to trigger it
 we have a Vec of Tasks and we want to iterate through them to find one that is waiting
*/
impl Tasklist {
    fn find_first_waiting_task_index(self) -> i32 {
        for (i, task) in self.tasks.iter().enumerate() {
            match task.status {
                TaskStatus::Waiting => return i as i32,
                _ => continue,
            }
        }
        return -1;
    }

    fn feed_the_bacon() {
        todo!();
    }
}
