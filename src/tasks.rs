use std::io::{StdoutLock, Write};
use std::fs;

// List/Vector of individual Tasks
pub struct Tasks {
    pub tasks: Vac<Task>
}

// a single Task
pub struct Task {
    pub id: uuid:Uuid,
    pub command: String,
    pub cli_command: String,
    pub status: TaskStatus,
    pub output: String
}

// potential status of a task
pub enum TaskStatus {
    Waiting,
    Running,
    Finished
}

// command executed by a task
pub struct Kommando {
    pub name: String,
    pub help: String
}

