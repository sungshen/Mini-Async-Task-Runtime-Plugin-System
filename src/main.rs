mod runtime;
mod plugin;

use crate::runtime::task::Task;
use crate::runtime::executor::Executor;
use crate::plugin::api::Api;


fn main() {
    let mut ex = Executor::new();
    let ta = Task::new(Box::new(||{ println!("Hello, world1");}));
    let tk = Task::new(Box::new(||{ println!("Hello, world2");}));

    ex.spawn(ta);
    ex.spawn(tk);
    ex.run();
}
