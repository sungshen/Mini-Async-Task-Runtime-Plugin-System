use crate::runtime::task::Task;

pub struct Executor {
    Queue: Vec<Task>,
}

impl Executor {
    pub fn new() -> Executor {
        Executor { Queue: Vec::new() }
    }
    pub fn spawn(&mut self, t: Task) {
        self.Queue.push(t);
    }
    pub fn run(self) -> Option<bool> {
        for q in self.Queue{
            let f :Box<dyn FnOnce()> = q.Job;
            f();
        }
    Some(true)
    }
}
