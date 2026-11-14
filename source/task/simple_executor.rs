use super::Task;
use alloc::collections::VecDeque;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

fn dummy_raw_waker() -> RawWaker {
    fn clone(_: *const ()) -> RawWaker {
        dummy_raw_waker()
    }
    fn no_operation(_: *const ()) -> () {}

    let virtual_method_table = &RawWakerVTable::new(clone, no_operation, no_operation, no_operation);
    RawWaker::new(0 as *const (), virtual_method_table)
}

fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}

pub struct SimpleExecutor {
    task_queue: VecDeque<Task>,
}

impl SimpleExecutor {
    pub fn new() -> Self {
        Self {
            task_queue: VecDeque::new(),
        }
    }

    pub fn run(&mut self) -> () {
        while let Some(mut task) = self.task_queue.pop_front() {
            let waker = dummy_waker();
            let mut context = Context::from_waker(&waker);

            match task.poll(&mut context) {
                | Poll::Pending => self.task_queue.push_back(task),
                | Poll::Ready(_) => {}
            }
        }
    }

    pub fn spawn(&mut self, task: Task) -> () {
        self.task_queue.push_back(task);
    }
}
