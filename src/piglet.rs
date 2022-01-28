use crate::tasks;
use std::fmt;

pub struct Piglet {
    pub id: uuid::Uuid,
    pub call_home_addr: String,
    pub bind_port: u16,
    pub tasklist: tasks::Tasklist,
}

// this generates default values for the Piglet Struct ->
// it autopopulates all the fields that are not passed during instantiation
impl Default for Piglet {
    fn default() -> Self {
        let uid = uuid::Uuid::new_v4();

        Piglet {
            id: uid,
            call_home_addr: "127.0.0.1".to_string(),
            bind_port: 8080,
            tasklist: tasks::Tasklist::default(),
        }
    }
}

impl fmt::Display for Piglet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} http://{}:{} {:?}",
            self.id, self.call_home_addr, self.bind_port, self.tasklist.tasks
        )
    }
}

impl Piglet {
    pub fn add_task(&mut self, cmd: String) {
        let task = tasks::Task {
            command: cmd.to_string(),
            ..Default::default()
        };
        self.tasklist.tasks.push(task)
    }
}
