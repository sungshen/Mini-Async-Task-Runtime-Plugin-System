

pub struct Task {
    pub Job: Box<dyn FnOnce()>,
}
impl Task {
    pub fn new(T:Box<dyn FnOnce()>) -> Task{
        let a = Task{ Job: Box::new(T)};

        a
    }
}

    
