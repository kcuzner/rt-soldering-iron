//! Minimal RTOS, based loosly on kos-avr

use core::u32;

/// State of a task
enum TaskState<'a> {
    /// The task is not yet started
    Starting,
    /// The task is ready
    Ready,
    /// The task is blocked on a mutex
    Mutex(&'a Mutex),
    /// The task is blocked on a queue
    Queue(&'a Queue)
}

pub struct Task<'a> {
    func: &'static Fn(),
    next: Option<&'a mut Task<'a>>,
    state: TaskState<'a>,
    stack: *mut u32,
}

impl<'a> Task<'a> {
    /// Creates a new task
    pub fn new(f: &'static Fn(), sp: &mut [u32]) -> Task<'a> {
        Task {
            func: f,
            next: None,
            state: TaskState::Starting,
            stack: unsafe { sp[0] as *mut u32 }
        }
    }

    /// Looks down this task's next task pointer to find the next unblocked task.
    /// Does not check this task.
    fn get_next_unblocked(&self) -> &Option<&'a mut Task<'a>> {
        match &self.next {
            &None => &None,
            &Some(ref t) => match &t.state {
                &TaskState::Starting | &TaskState::Ready => &self.next,
                _ => t.get_next_unblocked()
            }
        }
    }

    fn dispatch(&mut self) {
    }
}

pub struct TaskGroup<'a> {
    head: Option<&'a mut Task<'a>>
}

impl<'a> TaskGroup<'a> {
    pub fn new() -> TaskGroup<'a> {
        TaskGroup {
            head: None
        }
    }
    pub fn add_task(mut self, tcb: &'a mut Task<'a>) -> TaskGroup<'a> {
        match self.head {
            None => { self.head = Some(tcb); self },
            Some(h) => { tcb.next = Some(h); self.head = Some(tcb); self }
        }
    }
    pub fn run(mut self) {
    }

    fn get_next_task(&self) -> &Option<&'a mut Task<'a>> {
        match &self.head {
            &None => &None,
            &Some(ref t) => match &t.state {
                &TaskState::Starting | &TaskState::Ready => &self.head,
                _ => t.get_next_unblocked()
            }
        }
    }
}

pub struct Mutex {
}

pub struct Queue {
}

