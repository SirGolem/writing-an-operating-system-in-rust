pub mod executor;
pub mod keyboard;
pub mod simple_executor;

use alloc::boxed::Box;
use core::{
    pin::Pin,
    sync::atomic::{AtomicU64, Ordering},
    task::{Context, Poll},
};

pub struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
    identifier: TaskIdentifier,
}

impl Task {
    pub fn new(future: impl Future<Output = ()> + 'static) -> Self {
        Self {
            future: Box::pin(future),
            identifier: TaskIdentifier::new(),
        }
    }

    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct TaskIdentifier(u64);

impl TaskIdentifier {
    fn new() -> Self {
        static NEXT_TASK_IDENTIFIER: AtomicU64 = AtomicU64::new(0);
        Self(NEXT_TASK_IDENTIFIER.fetch_add(1, Ordering::Relaxed))
    }
}
