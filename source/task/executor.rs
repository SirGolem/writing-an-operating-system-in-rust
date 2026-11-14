use crate::task::{Task, TaskIdentifier};
use alloc::{collections::BTreeMap, sync::Arc, task::Wake};
use core::task::{Context, Poll, Waker};
use crossbeam_queue::ArrayQueue;
use x86_64::instructions::interrupts;

pub struct Executor {
    task_queue: Arc<ArrayQueue<TaskIdentifier>>,
    tasks: BTreeMap<TaskIdentifier, Task>,
    waker_cache: BTreeMap<TaskIdentifier, Waker>,
}

impl Executor {
    fn halt_if_idle(&self) -> () {
        interrupts::disable();

        if self.task_queue.is_empty() {
            interrupts::enable_and_hlt();
        } else {
            interrupts::enable();
        }
    }

    pub fn new() -> Self {
        Self {
            task_queue: Arc::new(ArrayQueue::new(128)),
            tasks: BTreeMap::new(),
            waker_cache: BTreeMap::new(),
        }
    }

    pub fn run(&mut self) -> ! {
        loop {
            self.run_ready_tasks();
            self.halt_if_idle();
        }
    }

    fn run_ready_tasks(&mut self) -> () {
        while let Some(task_identifier) = self.task_queue.pop() {
            let task = match self.tasks.get_mut(&task_identifier) {
                | None => continue,
                | Some(task) => task,
            };
            let waker = self
                .waker_cache
                .entry(task_identifier)
                .or_insert_with(|| TaskWaker::new(task_identifier, self.task_queue.clone()));
            let mut context = Context::from_waker(waker);

            match task.poll(&mut context) {
                | Poll::Pending => {}
                | Poll::Ready(_) => {
                    self.tasks.remove(&task_identifier);
                    self.waker_cache.remove(&task_identifier);
                }
            }
        }
    }

    pub fn spawn(&mut self, task: Task) -> () {
        let task_identifier = task.identifier;
        if self.tasks.insert(task.identifier, task).is_some() {
            panic!("Task with the same identifier already exists.");
        }
        self.task_queue.push(task_identifier).expect("Task queue is full");
    }
}

struct TaskWaker {
    task_identifier: TaskIdentifier,
    task_queue: Arc<ArrayQueue<TaskIdentifier>>,
}

impl TaskWaker {
    fn new(task_identifier: TaskIdentifier, task_queue: Arc<ArrayQueue<TaskIdentifier>>) -> Waker {
        Waker::from(Arc::new(Self {
            task_identifier,
            task_queue,
        }))
    }

    fn wake_task(&self) -> () {
        self.task_queue.push(self.task_identifier).expect("Task queue is full");
    }
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) -> () {
        self.wake_task();
    }

    fn wake_by_ref(self: &Arc<Self>) -> () {
        self.wake_task();
    }
}
