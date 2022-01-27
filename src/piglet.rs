use crate::tasks;
use std::fmt;

pub struct Piglet {
    pub id: uuid::Uuid,
    pub bind_port: u16,
    pub call_home_addr: String,
    pub tasks: tasks::Tasklist,
}

// this generates default values for the Piglet Struct ->
// it autopopulates all the fields that are not passed during instantiation
impl Default for Piglet {
    fn default() -> Self {
        let uid = uuid::Uuid::new_v4();

        Piglet {
            id: uid,
            bind_port: 8080,
            call_home_addr: "127.0.0.1".to_string(),
            tasks: tasks::Tasklist::default(),
        }
    }
}

impl fmt::Display for Piglet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.id, self.bind_port, self.call_home_addr)
    }
}

impl Piglet {
    pub fn add_task(mut self, cmd: &str) {
        let task = tasks::Task {
            command: cmd.to_string(),
            ..Default::default()
        };
        self.tasks.tasks.push(task)
    }
}
